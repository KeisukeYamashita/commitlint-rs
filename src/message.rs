use crate::{
    git::{parse_commit_message, parse_subject},
    result::Result as LintResult,
};
use std::{collections::HashMap, fmt::Error};

/// Message represents a single commit message.
///
///
/// ```code
/// <type>[optional scope]: <description>
///
/// [optional body]
///
/// [optional footer(s)]
/// ```
///
#[derive(Clone, Debug)]
pub struct Message {
    /// Description part of the commit message.
    pub description: Option<String>,

    /// Raw commit message (or any input from stdin) including the body and footers.
    pub raw: String,

    /// Type part of the commit message.
    pub r#type: Option<String>,

    /// Scope part of the commit message.
    pub scope: Option<String>,

    /// Body part of the commit message.
    pub body: Option<String>,

    /// Footers part of the commit message.
    pub footers: Option<HashMap<String, String>>,
}

/// Message represents a commit message.
impl Message {
    /// Create a new Message.
    pub fn new(raw: String) -> Self {
        let (subject, body, footers) = parse_commit_message(&raw);
        match parse_subject(&subject) {
            Some((r#type, scope, description)) => Self {
                raw,
                description: Some(description),
                r#type: Some(r#type),
                scope,
                body,
                footers,
            },
            None => Self {
                raw,
                description: None,
                r#type: None,
                scope: None,
                body,
                footers,
            },
        }
    }

    /// Lint the raw commit message.
    pub async fn lint(&self) -> Result<LintResult, Error> {
        // TODO: Implement linting.
        Ok(LintResult {
            violations: Some(vec!["Hello".to_string()]),
        })
    }
}
