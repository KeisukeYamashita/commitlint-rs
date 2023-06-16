use crate::{message::Message, result::Violation, rule::Rule};
use serde::Deserialize;

use super::Level;

/// BodyEmpty represents the body-empty rule.
#[derive(Clone, Debug, Deserialize)]
pub struct BodyEmpty {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,
}

/// BodyEmpty represents the body-empty rule.
impl Rule for BodyEmpty {
    const NAME: &'static str = "body-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "body is empty".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.body.is_none() {
            return Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            });
        }

        None
    }
}

/// Default implementation of BodyEmpty.
impl Default for BodyEmpty {
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
    fn test_non_empty_body() {
        let rule = BodyEmpty::default();
        let message = Message {
            body: Some("Hello world".to_string()),
            description: Some("broadcast $destroy event on scope destruction".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction

Hello world"
                .to_string(),
            scope: Some("scope".to_string()),
            subject: Some("feat(scope): broadcast $destroy event on scope destruction".to_string()),
        };

        assert_eq!(rule.validate(&message).is_none(), true);
    }

    #[test]
    fn test_empty_body() {
        let rule = BodyEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert_eq!(violation.is_some(), true);
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.clone().unwrap().message,
            "body is empty".to_string()
        );
    }
}
