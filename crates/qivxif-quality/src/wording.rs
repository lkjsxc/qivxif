use crate::error::{QualityError, finish};
use std::{fs, path::Path};

const BANNED_WORDS: &[&str] = &["version", "versions", "versioned"];

pub fn check_wording() -> Result<(), QualityError> {
    let mut failures = Vec::new();
    check_markdown_tree(Path::new("docs"), &mut failures)?;
    for path in [Path::new("README.md"), Path::new("AGENTS.md")] {
        if path.exists() {
            check_file(path, &mut failures)?;
        }
    }
    finish(failures, "wording ok")
}

fn check_markdown_tree(dir: &Path, failures: &mut Vec<String>) -> Result<(), QualityError> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            check_markdown_tree(&path, failures)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            check_file(&path, failures)?;
        }
    }
    Ok(())
}

fn check_file(path: &Path, failures: &mut Vec<String>) -> Result<(), QualityError> {
    let contents = fs::read_to_string(path)?;
    for (line_number, line) in contents.lines().enumerate() {
        let lower = line.to_ascii_lowercase();
        for word in BANNED_WORDS {
            if contains_word(&lower, word) {
                failures.push(format!(
                    "{}:{} banned word `{word}`",
                    path.display(),
                    line_number + 1
                ));
            }
        }
        if contains_release_label(&lower) {
            failures.push(format!(
                "{}:{} banned release label",
                path.display(),
                line_number + 1
            ));
        }
    }
    Ok(())
}

fn contains_word(line: &str, word: &str) -> bool {
    line.split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|part| part == word)
}

fn contains_release_label(line: &str) -> bool {
    line.split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(is_release_label)
}

fn is_release_label(part: &str) -> bool {
    let Some(rest) = part.strip_prefix('v') else {
        return false;
    };
    !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit())
}
