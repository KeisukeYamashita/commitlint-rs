use crate::{message::Message, result::Violation, rule::Rule};
use serde::Deserialize;

use super::Level;

/// DescriptionEmpty represents the subject-empty rule.
#[derive(Clone, Debug, Deserialize)]
pub struct DescriptionEmpty {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,
}

/// DescriptionEmpty represents the description-empty rule.
impl Rule for DescriptionEmpty {
    const NAME: &'static str = "description-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "description is empty or missing space in the beginning".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.description.is_none() {
            return Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            });
        }

        None
    }
}

/// Default implementation of DescriptionEmpty.
impl Default for DescriptionEmpty {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_description() {
        let rule = DescriptionEmpty::default();
        let message = Message {
            body: None,
            description: Some("broadcast $destroy event on scope destruction".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: Some("scope".to_string()),
        };

        assert_eq!(rule.validate(&message).is_none(), true);
    }

    #[test]
    fn test_empty_description() {
        let rule = DescriptionEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "(scope):".to_string(),
            scope: Some("scope".to_string()),
        };

        let violation = rule.validate(&message);
        assert_eq!(violation.is_some(), true);
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.clone().unwrap().message,
            "description is empty".to_string()
        );
    }
}
