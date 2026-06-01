use crate::error::{QualityError, finish};
use std::{
    fs,
    path::{Component, Path},
};

const TERMS: &[&str] = &["kjxlkj", "catppuccin", "frappe", "frappé"];

pub fn check_public_names() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    visit(Path::new("."), &mut failures)?;
    finish(failures, "public names ok")
}

fn visit(path: &Path, failures: &mut Vec<String>) -> Result<(), QualityError> {
    if skip_path(path) {
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
    for (line_index, line) in contents.lines().enumerate() {
        let lower = line.to_lowercase();
        for term in TERMS {
            if lower.contains(term) {
                failures.push(format!(
                    "{}:{} contains retired public name",
                    path.display(),
                    line_index + 1
                ));
            }
        }
    }
    Ok(())
}

fn skip_path(path: &Path) -> bool {
    if path.ends_with("crates/qivxif-quality/src/public_names.rs") {
        return true;
    }
    path.components().any(skip_component)
}

fn skip_component(component: Component<'_>) -> bool {
    let Component::Normal(value) = component else {
        return false;
    };
    matches!(
        value.to_str(),
        Some(".git") | Some("target") | Some("tmp") | Some("dist") | Some("node_modules")
    )
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
