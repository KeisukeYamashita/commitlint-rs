use std::fmt::Error;

use crate::result::Result as Res;

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
    pub async fn lint(&self) -> Result<Res, Error> {
        // TODO: Implement linting.
        println!("Linting: {}", self.raw);
        Ok(Res {
            violations: Some(vec![]),
        })
    }
}
