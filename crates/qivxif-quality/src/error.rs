use thiserror::Error;

#[derive(Debug, Error)]
pub enum QualityError {
    #[error("quality check failed:\n{0}")]
    Failed(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub(crate) fn finish(failures: Vec<String>, ok: &str) -> Result<(), QualityError> {
    if failures.is_empty() {
        println!("{ok}");
        Ok(())
    } else {
        Err(QualityError::Failed(failures.join("\n")))
    }
}
