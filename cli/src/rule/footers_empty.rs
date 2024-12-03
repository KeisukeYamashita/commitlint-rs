use crate::{make_rule, message::Message, result::Violation, rule::Rule};

use super::Level;
make_rule! {
    FootersEmpty,
}

/// FooterEmpty represents the footer-empty rule.
impl Rule for FootersEmpty {
    const NAME: &'static str = "footers-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "footers are empty".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.footers.is_none() {
            return Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            });
        }

        None
    }
}

/// Default implementation of FooterEmpty.
impl Default for FootersEmpty {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_non_empty_footer() {
        let rule = FootersEmpty::default();

        let mut f = HashMap::new();
        f.insert("Link".to_string(), "hello".to_string());

        let message = Message {
            body: Some("Hello world".to_string()),
            description: Some("broadcast $destroy event on scope destruction".to_string()),
            footers: Some(f),
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction

Hello world

Link: hello"
                .to_string(),
            scope: Some("scope".to_string()),
            subject: Some("feat(scope): broadcast $destroy event on scope destruction".to_string()),
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_empty_footer() {
        let rule = FootersEmpty::default();
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
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(violation.unwrap().message, "footers are empty".to_string());
    }
}
