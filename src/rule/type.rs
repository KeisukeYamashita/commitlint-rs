use crate::{message::Message, result::Violation, rule::Rule};
use serde::Deserialize;

use super::Level;

/// Type represents the subject-empty rule.
#[derive(Clone, Debug, Deserialize)]
pub struct Type {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,

    /// Options represents the options of the rule.
    /// If the option is empty, it means that no Type is allowed.
    options: Vec<String>,
}

/// Type represents the type rule.
impl Rule for Type {
    const NAME: &'static str = "type";
    const LEVEL: Level = Level::Error;

    fn message(&self, message: &Message) -> String {
        format!(
            "type {} is not allowed. Only {:?} are allowed",
            message.r#type.as_ref().unwrap(),
            self.options
        )
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if let Some(t) = &message.r#type {
            if self.options.contains(t) {
                return None;
            }
        }

        Some(Violation {
            level: self.level.unwrap_or(Self::LEVEL),
            message: self.message(message),
        })
    }
}

/// Default implementation of Type.
impl Default for Type {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
            options: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_type() {
        let mut rule = Type::default();
        rule.options = vec!["doc".to_string(), "feat".to_string()];

        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: None,
        };

        assert_eq!(rule.validate(&message).is_none(), true);
    }

    #[test]
    fn test_invalid_type() {
        let mut rule = Type::default();
        rule.options = vec!["doc".to_string(), "feat".to_string()];

        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("invalid".to_string()),
            raw: "invalid(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: None,
        };

        let violation = rule.validate(&message);
        assert_eq!(violation.is_some(), true);
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.clone().unwrap().message,
            "type invalid is not allowed. Only [\"doc\", \"feat\"] are allowed".to_string()
        );
    }

    #[test]
    fn test_no_options() {
        let rule = Type::default();

        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("invalid".to_string()),
            raw: "invalid(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: None,
        };

        let violation = rule.validate(&message);
        assert_eq!(violation.is_some(), true);
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.clone().unwrap().message,
            "type invalid is not allowed. Only [] are allowed".to_string()
        );
    }
}
