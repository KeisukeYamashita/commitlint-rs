use super::Level;
use crate::{make_rule, message::Message, result::Violation, rule::Rule};

make_rule! {
    ScopeEmpty,
}

/// ScopeEmpty represents the scope-empty rule.
impl Rule for ScopeEmpty {
    const NAME: &'static str = "scope-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "scope is empty".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.scope.is_none() {
            return Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            });
        }

        None
    }
}

/// Default implementation of ScopeEmpty.
impl Default for ScopeEmpty {
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
    fn test_non_empty_subject() {
        let rule = ScopeEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_no_subject() {
        let rule = ScopeEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat: broadcast $destroy event on scope destruction".to_string(),
            scope: None,
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(violation.unwrap().message, "scope is empty".to_string());
    }

    #[test]
    fn test_empty_subject() {
        let rule = ScopeEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(): broadcast $destroy event on scope destruction".to_string(),
            scope: None,
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(violation.unwrap().message, "scope is empty".to_string());
    }
}
