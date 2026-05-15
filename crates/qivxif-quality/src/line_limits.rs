use crate::error::{QualityError, finish};
use serde::Serialize;
use std::{fs, path::Path};

const DOC_LIMIT: usize = 300;
const SRC_LIMIT: usize = 200;

#[derive(Serialize)]
struct LineReport {
    status: &'static str,
    docs_max: usize,
    src_max: usize,
    violations: Vec<String>,
}

pub fn check_lines() -> Result<(), QualityError> {
    let mut report = LineReport {
        status: "pass",
        docs_max: 0,
        src_max: 0,
        violations: Vec::new(),
    };
    check_tree(Path::new("docs"), DOC_LIMIT, true, &mut report)?;
    check_tree(Path::new("apps"), SRC_LIMIT, false, &mut report)?;
    check_tree(Path::new("crates"), SRC_LIMIT, false, &mut report)?;
    check_tree(Path::new("scripts"), SRC_LIMIT, false, &mut report)?;
    check_tree(Path::new("docker"), SRC_LIMIT, false, &mut report)?;
    check_tree(Path::new("config"), SRC_LIMIT, false, &mut report)?;
    check_root_support_files(SRC_LIMIT, &mut report)?;
    if !report.violations.is_empty() {
        report.status = "fail";
    }
    println!("{}", serde_json::to_string(&report)?);
    finish(report.violations, "line limits ok")
}

fn check_tree(
    dir: &Path,
    limit: usize,
    docs: bool,
    report: &mut LineReport,
) -> Result<(), QualityError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            check_tree(&path, limit, docs, report)?;
        } else if should_check(&path, docs) {
            check_file(&path, limit, docs, report)?;
        }
    }
    Ok(())
}

fn check_file(
    path: &Path,
    limit: usize,
    docs: bool,
    report: &mut LineReport,
) -> Result<(), QualityError> {
    let lines = fs::read_to_string(path)?.lines().count();
    if docs {
        report.docs_max = report.docs_max.max(lines);
    } else {
        report.src_max = report.src_max.max(lines);
    }
    if lines > limit {
        report.violations.push(format!(
            "{} has {lines} lines; limit {limit}",
            path.display()
        ));
    }
    Ok(())
}

fn should_check(path: &Path, docs: bool) -> bool {
    let ext = path.extension().and_then(|ext| ext.to_str());
    if docs {
        return ext == Some("md");
    }
    matches!(
        ext,
        Some("rs") | Some("sh") | Some("toml") | Some("yml") | Some("yaml")
    ) || path.file_name().and_then(|name| name.to_str()) == Some("Dockerfile")
        || ext == Some("Dockerfile")
}

fn check_root_support_files(limit: usize, report: &mut LineReport) -> Result<(), QualityError> {
    for name in [
        "Cargo.toml",
        "Dockerfile",
        "Dockerfile.verify",
        "docker-compose.yml",
        "docker-compose.verify.yml",
        "rust-toolchain.toml",
    ] {
        let path = Path::new(name);
        if path.exists() {
            check_file(path, limit, false, report)?;
        }
    }
    Ok(())
}
