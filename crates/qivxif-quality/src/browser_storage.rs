use crate::error::{QualityError, finish};
use std::{fs, path::Path};

pub fn check_browser_storage_boundaries() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    for path in [
        Path::new("apps/qivxif-web/package.json"),
        Path::new("apps/qivxif-web/src"),
    ] {
        visit(path, &mut failures)?;
    }
    finish(failures, "browser storage boundaries ok")
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
        let lower = text.to_ascii_lowercase();
        if lower.contains("dexie") {
            failures.push(format!("{}:{} contains Dexie", path.display(), line + 1));
        }
        if component_opens_storage(path, &lower) {
            failures.push(format!(
                "{}:{} component opens raw storage",
                path.display(),
                line + 1
            ));
        }
    }
    Ok(())
}

fn component_opens_storage(path: &Path, lower: &str) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("svelte")
        && (lower.contains("indexeddb") || lower.contains("opfs") || lower.contains("new worker"))
}

fn skip(path: &Path) -> bool {
    let value = path.to_string_lossy();
    value.contains("/dist") || value.contains("/node_modules") || value.contains("/.svelte-kit")
}

fn text_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("json") | Some("ts") | Some("js") | Some("svelte")
    )
}
