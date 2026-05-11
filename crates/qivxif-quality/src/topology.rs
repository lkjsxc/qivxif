use crate::error::{QualityError, finish};
use std::{fs, path::Path};

pub fn validate_docs_topology() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    visit_dirs(Path::new("docs"), &mut |dir| {
        validate_dir(dir, &mut failures)
    })?;
    finish(failures, "docs topology ok")
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
