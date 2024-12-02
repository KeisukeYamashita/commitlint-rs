use crate::{make_length_rule, message::Message, result::Violation, rule::Rule};

use super::Level;

make_length_rule! {
    DescriptionMaxLength,
    "description"
}

/// DescriptionMaxLength represents the description-max-length rule.
impl Rule for DescriptionMaxLength {
    const NAME: &'static str = "description-max-length";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!("description is longer than {} characters", self.length)
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if let Some(desc) = &message.description {
            if desc.len() >= self.length {
                return Some(Violation {
                    level: self.level.unwrap_or(Self::LEVEL),
                    message: self.message(message),
                });
            }
        }

        None
    }
}

/// Default implementation of DescriptionMaxLength.
impl Default for DescriptionMaxLength {
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
    fn test_long_description() {
        let rule = DescriptionMaxLength {
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
    fn test_empty_description() {
        let rule = DescriptionMaxLength {
            length: 10, // Short length for testing
            ..Default::default()
        };
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope)".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_short_description() {
        let rule = DescriptionMaxLength {
            length: 10, // Short length for testing
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
            format!("description is longer than {} characters", rule.length)
        );
    }
}
