use crate::{make_rule, message::Message, result::Violation, rule::Rule};

use super::Level;
make_rule! {
    TypeEmpty,
}

/// TypeEmpty represents the type-empty rule.
impl Rule for TypeEmpty {
    const NAME: &'static str = "type-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "type is empty".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.r#type.is_none() || message.r#type.as_ref().unwrap().is_empty() {
            return Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            });
        }

        None
    }
}

/// Default implementation of TypeEmpty.
impl Default for TypeEmpty {
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
    fn test_non_empty_type() {
        let rule = TypeEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: None,
            subject: None,
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_empty_type() {
        let rule = TypeEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: None,
            raw: "(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: None,
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(violation.unwrap().message, "type is empty".to_string());
    }
}
