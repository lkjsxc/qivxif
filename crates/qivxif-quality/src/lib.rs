use serde::Serialize;
use std::{fs, path::Path};
use thiserror::Error;

const DOC_LIMIT: usize = 300;
const SRC_LIMIT: usize = 200;

#[derive(Debug, Error)]
pub enum QualityError {
    #[error("quality check failed:\n{0}")]
    Failed(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Serialize)]
struct LineReport {
    status: &'static str,
    docs_max: usize,
    src_max: usize,
    violations: Vec<String>,
}

pub fn validate_docs_topology() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    visit_dirs(Path::new("docs"), &mut |dir| {
        validate_dir(dir, &mut failures)
    })?;
    finish(failures, "docs topology ok")
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
    if !report.violations.is_empty() {
        report.status = "fail";
    }
    println!("{}", serde_json::to_string(&report)?);
    finish(report.violations, "line limits ok")
}

fn validate_dir(dir: &Path, failures: &mut Vec<String>) -> Result<(), QualityError> {
    let readme = dir.join("README.md");
    if !readme.exists() {
        failures.push(format!("missing README.md: {}", dir.display()));
        return Ok(());
    }
    let child_count = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name() != "README.md")
        .count();
    if child_count < 2 {
        failures.push(format!("needs >=2 children: {}", dir.display()));
    }
    validate_readme_index(dir, &readme, failures)
}

fn validate_readme_index(
    dir: &Path,
    readme: &Path,
    failures: &mut Vec<String>,
) -> Result<(), QualityError> {
    let contents = fs::read_to_string(readme)?;
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let Some(target) = child_link_target(&path) else {
            continue;
        };
        if !contents.contains(&format!("]({target})")) {
            failures.push(format!(
                "README missing child link {target}: {}",
                readme.display()
            ));
        }
    }
    Ok(())
}

fn child_link_target(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    if name == "README.md" {
        return None;
    }
    if path.is_dir() {
        return Some(format!("{name}/README.md"));
    }
    if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
        return Some(name.to_string());
    }
    None
}

fn visit_dirs(
    dir: &Path,
    action: &mut dyn FnMut(&Path) -> Result<(), QualityError>,
) -> Result<(), QualityError> {
    action(dir)?;
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            visit_dirs(&path, action)?;
        }
    }
    Ok(())
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
    matches!(ext, Some("rs") | Some("sh"))
}

fn finish(failures: Vec<String>, ok: &str) -> Result<(), QualityError> {
    if failures.is_empty() {
        println!("{ok}");
        Ok(())
    } else {
        Err(QualityError::Failed(failures.join("\n")))
    }
}
