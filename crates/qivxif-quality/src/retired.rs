use crate::error::{QualityError, finish};
use std::{fs, path::Path};

const TERMS: &[&str] = &[
    "qivxif-superapp",
    "qivxif-shell",
    "native shell",
    "native desktop",
    "egui",
    "winit",
    "wgpu",
    "wry",
    "xvfb",
    "superapp-smoke",
    "smoke-native",
    "workspace_json",
    "settings_toml",
];

pub fn check_retired_canon() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    visit(Path::new("."), &mut failures)?;
    finish(failures, "retired canon ok")
}

fn visit(path: &Path, failures: &mut Vec<String>) -> Result<(), QualityError> {
    if skip(path) {
        return Ok(());
    }
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            visit(&entry?.path(), failures)?;
        }
    } else if text_file(path) {
        check_file(path, failures)?;
    }
    Ok(())
}

fn check_file(path: &Path, failures: &mut Vec<String>) -> Result<(), QualityError> {
    let contents = fs::read_to_string(path)?;
    let lower = contents.to_ascii_lowercase();
    for term in TERMS {
        if lower.contains(term) {
            failures.push(format!("{} contains retired term `{term}`", path.display()));
        }
    }
    Ok(())
}

fn skip(path: &Path) -> bool {
    let value = path.to_string_lossy();
    value.contains("/.git")
        || value.contains("/target")
        || value.contains("/tmp")
        || value.contains("/dist")
        || value.contains("/node_modules")
        || value.contains("/.svelte-kit")
        || value.contains("Cargo.lock")
        || value.ends_with("package-lock.json")
        || value.ends_with("docs/decisions/rejected.md")
        || value.ends_with("crates/qivxif-quality/src/retired.rs")
}

fn text_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("md")
            | Some("rs")
            | Some("toml")
            | Some("yml")
            | Some("yaml")
            | Some("sh")
            | Some("json")
            | Some("html")
            | Some("css")
            | Some("ts")
            | Some("js")
            | Some("mjs")
    ) || path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "Dockerfile" || name.starts_with("Dockerfile."))
}
