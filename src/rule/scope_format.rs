use crate::{message::Message, result::Violation, rule::Rule};
use serde::{Deserialize, Serialize};

use super::Level;

/// ScopeFormat represents the scope-format rule.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScopeFormat {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,

    /// Format represents the format of the scope.
    format: Option<String>,
}

/// ScopeFormat represents the scope-format rule.
impl Rule for ScopeFormat {
    const NAME: &'static str = "scope-format";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!(
            "scope format does not match format: {}",
            self.format.as_ref().unwrap()
        )
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if let Some(format) = &self.format {
            let regex = regex::Regex::new(format).unwrap();
            if !regex.is_match(&message.scope.as_ref().unwrap()) {
                return Some(Violation {
                    level: self.level.unwrap_or(Self::LEVEL),
                    message: self.message(message),
                });
            }
        }

        None
    }
}

/// Default implementation of ScopeFormat.
impl Default for ScopeFormat {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
            format: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_description_format() {
        let mut rule = ScopeFormat::default();
        rule.format = Some(r"^[a-z].*".to_string());

        let message = Message {
            body: None,
            description: Some("Add new flag".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): Add new flag".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_valid_description_format() {
        let mut rule = ScopeFormat::default();
        rule.format = Some(r"^[a-z].*".to_string());

        let message = Message {
            body: None,
            description: Some("Add new flag".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(Scope): Add new flag".to_string(),
            scope: Some("Scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            "scope format does not match format: ^[a-z].*".to_string()
        );
    }
}
