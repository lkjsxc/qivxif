use crate::error::{QualityError, finish};
use std::{collections::BTreeSet, fs};

pub fn check_redb_tables_match_docs() -> Result<(), QualityError> {
    let docs = fs::read_to_string("docs/architecture/storage/redb-layout.md")?;
    let source = fs::read_to_string("crates/qivxif-store-redb/src/tables.rs")?;
    let documented = documented_tables(&docs);
    let registered = registered_tables(&source);
    let mut failures = set_failures(&documented, &registered);
    if failures.is_empty() && documented != registered {
        failures.push("documented table order differs from tables::ALL".to_owned());
    }
    finish(failures, "redb tables match docs")
}

fn set_failures(documented: &[String], registered: &[String]) -> Vec<String> {
    let docs: BTreeSet<_> = documented.iter().cloned().collect();
    let code: BTreeSet<_> = registered.iter().cloned().collect();
    let mut failures = Vec::new();
    for table in docs.difference(&code) {
        failures.push(format!("documented table is not registered: {table}"));
    }
    for table in code.difference(&docs) {
        failures.push(format!("registered table is not documented: {table}"));
    }
    failures
}

fn documented_tables(contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter_map(|line| {
            let cells: Vec<_> = line.split('|').map(str::trim).collect();
            if cells.len() < 5 {
                return None;
            }
            let table = code_cell(cells[1])?;
            (!matches!(table.as_str(), "Table")).then_some(table)
        })
        .collect()
}

fn registered_tables(contents: &str) -> Vec<String> {
    let mut in_all = false;
    let mut tables = Vec::new();
    for line in contents.lines().map(str::trim) {
        if line.starts_with("pub const ALL") {
            in_all = true;
            continue;
        }
        if in_all && line == "];" {
            break;
        }
        if in_all && let Some(table) = quoted_tuple_head(line) {
            tables.push(table);
        }
    }
    tables
}

fn quoted_tuple_head(line: &str) -> Option<String> {
    let rest = line.strip_prefix("(\"")?;
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

fn code_cell(cell: &str) -> Option<String> {
    let start = cell.find('`')? + 1;
    let end = cell[start..].find('`')? + start;
    Some(cell[start..end].to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_documented_table_names() {
        let tables = documented_tables("| `users` | `UserId` | value | owner |\n");
        assert_eq!(tables, vec!["users"]);
    }

    #[test]
    fn parses_registered_table_names_from_all() {
        let source = "pub const ALL: &[&str] = &[\n    (\"users\", USERS),\n];";
        assert_eq!(registered_tables(source), vec!["users"]);
    }
}
