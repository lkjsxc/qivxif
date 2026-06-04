use crate::error::{QualityError, finish};
use std::{fs, path::Path};

const TERMS: &[&str] = &[
    "TODO",
    "FIXME",
    "placeholder",
    "mock",
    "dummy",
    "temporary",
    "not implemented",
    "unimplemented!",
    "todo!",
];

pub fn check_placeholders() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    visit(Path::new("."), &mut failures)?;
    finish(failures, "implementation markers ok")
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
    for (line, text) in contents.lines().enumerate() {
        let text = text
            .replace("check-placeholders", "quality-command")
            .replace("CheckPlaceholders", "QualityCommand")
            .replace("check_placeholders", "quality_command");
        for term in TERMS {
            if text.contains(term) {
                failures.push(format!("{}:{} contains `{term}`", path.display(), line + 1));
            }
        }
        if text.contains("panic!(\"not implemented\")") {
            failures.push(format!(
                "{}:{} contains panic marker",
                path.display(),
                line + 1
            ));
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
        || value.ends_with("docs/decisions/rejected.md")
        || value.ends_with("crates/qivxif-quality/src/markers.rs")
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
    )
}
