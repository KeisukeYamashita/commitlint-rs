use crate::result::Result as LintResult;
use std::fmt::Error;

/// Message represents a single commit message.
#[derive(Debug)]
pub struct Message {
    /// Raw commit message (or any input from stdin).
    pub raw: String,
}

impl Message {
    /// Create a new Message.
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    /// Lint the raw commit message.
    pub async fn lint(&self) -> Result<LintResult, Error> {
        // TODO: Implement linting.
        println!("Linting: {}", self.raw);
        Ok(LintResult {
            violations: Some(vec!["Hello".to_string()]),
        })
    }
}
