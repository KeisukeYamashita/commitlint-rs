use crate::{make_rule, message::Message, result::Violation, rule::Rule};

use super::Level;
make_rule! {
    DescriptionEmpty,
}

/// DescriptionEmpty represents the description-empty rule.
impl Rule for DescriptionEmpty {
    const NAME: &'static str = "description-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "description is empty or missing space in the beginning".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        match message.description {
            None => Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            }),
            Some(ref desc) if desc.is_empty() => Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            }),
            _ => None,
        }
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
            subject: None,
        };

        assert!(rule.validate(&message).is_none());
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
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            "description is empty or missing space in the beginning".to_string()
        );
    }

    #[test]
    fn test_blank_description() {
        let rule = DescriptionEmpty::default();
        let message = Message {
            body: None,
            description: Some("".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "(scope):".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            "description is empty or missing space in the beginning".to_string()
        );
    }
}
