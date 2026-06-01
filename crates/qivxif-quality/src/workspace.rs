use crate::error::{QualityError, finish};
use std::fs;

pub fn check_workspace_matches_docs() -> Result<(), QualityError> {
    let docs = fs::read_to_string("docs/repository/layout/workspace-layout.md")?;
    let cargo = fs::read_to_string("Cargo.toml")?;
    let documented = documented_members(&docs);
    let mut failures = Vec::new();
    for member in documented {
        if !cargo.contains(&format!("\"{member}\"")) {
            failures.push(format!(
                "Cargo workspace missing documented member `{member}`"
            ));
        }
    }
    finish(failures, "workspace matches docs")
}

fn documented_members(contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter_map(|line| {
            let value = line.trim();
            let start = value.find('`')?;
            let end = value[start + 1..].find('`')? + start + 1;
            let member = &value[start + 1..end];
            (member.starts_with("apps/") || member.starts_with("crates/"))
                .then(|| member.to_owned())
        })
        .collect()
}
