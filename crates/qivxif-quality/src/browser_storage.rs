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
            push(failures, path, line, "contains Dexie");
        }
        if lower.contains("indexeddb.open") {
            push(failures, path, line, "opens IndexedDB");
        }
        if component_opens_storage(path, &lower) {
            push(failures, path, line, "component opens raw storage");
        }
        if raw_sql_outside_worker(path, &lower) {
            push(
                failures,
                path,
                line,
                "contains raw SQL outside storage worker",
            );
        }
        if direct_worker_message_outside_client(path, &lower) {
            push(
                failures,
                path,
                line,
                "uses direct worker messages outside storage client",
            );
        }
    }
    Ok(())
}

fn component_opens_storage(path: &Path, lower: &str) -> bool {
    is_svelte(path)
        && (lower.contains("indexeddb")
            || lower.contains("opfs")
            || lower.contains("new worker")
            || lower.contains("postmessage"))
}

fn raw_sql_outside_worker(path: &Path, lower: &str) -> bool {
    !allowed_sql_file(path)
        && [
            "create table",
            "select json",
            "select count",
            "insert into",
            "delete from",
            "pragma ",
            "begin immediate",
            "commit",
            "rollback",
        ]
        .iter()
        .any(|needle| lower.contains(needle))
}

fn direct_worker_message_outside_client(path: &Path, lower: &str) -> bool {
    !allowed_worker_message_file(path)
        && (lower.contains("new worker")
            || lower.contains("postmessage(")
            || lower.contains("addeventlistener(\"message\"")
            || lower.contains("addeventlistener('message'"))
}

fn allowed_sql_file(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some("sqlite-schema.ts") | Some("sqlite-statements.ts") | Some("worker-runtime.ts")
    )
}

fn allowed_worker_message_file(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some("sqlite-worker-client.ts") | Some("sqlite.worker.ts")
    )
}

fn push(failures: &mut Vec<String>, path: &Path, line: usize, reason: &str) {
    failures.push(format!("{}:{} {}", path.display(), line + 1, reason));
}

fn is_svelte(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("svelte")
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
