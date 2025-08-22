use crate::{message::Message, result::Violation, rule::Rule};
use serde::{Deserialize, Serialize};

use super::Level;

/// ScopeMaxLength represents the description-max-length rule.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ScopeMaxLength {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,

    /// Length represents the maximum length of the scope.
    length: usize,
}

/// ScopeMaxLength represents the scope-max-length rule.
impl Rule for ScopeMaxLength {
    const NAME: &'static str = "scope-max-length";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!("scope is longer than {} characters", self.length)
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        match &message.scope {
            Some(scope) => {
                if scope.len() >= self.length {
                    return Some(Violation {
                        level: self.level.unwrap_or(Self::LEVEL),
                        message: self.message(message),
                    });
                }
            }
            None => {
                return Some(Violation {
                    level: self.level.unwrap_or(Self::LEVEL),
                    message: self.message(message),
                })
            }
        }

        None
    }
}

/// Default implementation of ScopeMaxLength.
impl Default for ScopeMaxLength {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
            length: 72,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_scope() {
        let rule = ScopeMaxLength {
            length: usize::MAX, // Long length for testing
            ..Default::default()
        };
        let message = Message {
            body: None,
            description: Some("desc".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): desc".to_string(),
            scope: Some("scope".to_string()),
            subject: Some("feat(scope): desc".to_string()),
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_no_scope() {
        let rule = ScopeMaxLength {
            length: usize::MAX, // Long length for testing
            ..Default::default()
        };
        let message = Message {
            body: None,
            description: Some("desc".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): desc".to_string(),
            scope: None,
            subject: Some("feat(scope): desc".to_string()),
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_short_scope() {
        let rule = ScopeMaxLength {
            length: 3, // Short length for testing
            ..Default::default()
        };
        let message = Message {
            body: None,
            description: Some("feat(scope): I'm long description".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): I'm long description".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            format!("scope is longer than {} characters", rule.length)
        );
    }
}
