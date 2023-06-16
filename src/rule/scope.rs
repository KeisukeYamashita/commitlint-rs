use crate::{message::Message, result::Violation, rule::Rule};
use serde::Deserialize;

use super::Level;

/// Scope represents the subject-empty rule.
#[derive(Clone, Debug, Deserialize)]
pub struct Scope {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,

    /// Options represents the options of the rule.
    /// If the option is empty, it means that no scope is allowed.
    options: Vec<String>,
}

/// Scope represents the scope rule.
impl Rule for Scope {
    const NAME: &'static str = "scope";
    const LEVEL: Level = Level::Error;

    fn message(&self, message: &Message) -> String {
        format!(
            "scope {} is not allowed. Only {:?} are allowed",
            message.scope.as_ref().unwrap(),
            self.options
        )
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if let Some(scope) = &message.scope {
            if self.options.contains(scope) {
                return None;
            }
        }

        Some(Violation {
            level: self.level.unwrap_or(Self::LEVEL),
            message: self.message(message),
        })
    }
}

/// Default implementation of Scope.
impl Default for Scope {
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
    fn test_valid_scope() {
        let mut rule = Scope::default();
        rule.options = vec!["api".to_string(), "web".to_string()];

        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(web): broadcast $destroy event on scope destruction".to_string(),
            scope: Some("web".to_string()),
            subject: None,
        };

        assert_eq!(rule.validate(&message).is_none(), true);
    }

    #[test]
    fn test_invalid_scope() {
        let mut rule = Scope::default();
        rule.options = vec!["api".to_string(), "web".to_string()];

        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(invalid): broadcast $destroy event on scope destruction".to_string(),
            scope: Some("invalid".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert_eq!(violation.is_some(), true);
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.clone().unwrap().message,
            "scope invalid is not allowed. Only [\"api\", \"web\"] are allowed".to_string()
        );
    }

    #[test]
    fn test_no_options() {
        let rule = Scope::default();

        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(invalid): broadcast $destroy event on scope destruction".to_string(),
            scope: Some("invalid".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert_eq!(violation.is_some(), true);
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.clone().unwrap().message,
            "scope invalid is not allowed. Only [] are allowed".to_string()
        );
    }
}
