use crate::{make_length_rule, message::Message, result::Violation, rule::Rule};

use super::Level;
make_length_rule! {
    BodyMaxLength,
    "body"
}

/// BodyMaxLength represents the body-max-length rule.
impl Rule for BodyMaxLength {
    const NAME: &'static str = "body-max-length";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!("body is longer than {} characters", self.length)
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        match &message.body {
            Some(body) => {
                if body.len() >= self.length {
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

/// Default implementation of BodyMaxLength.
impl Default for BodyMaxLength {
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
    fn test_long_body() {
        let rule = BodyMaxLength {
            length: usize::MAX, // Long length for testing
            ..Default::default()
        };
        let message = Message {
            body: Some("Hello world".to_string()),
            description: Some("broadcast $destroy event on scope destruction".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction

Hey!"
                .to_string(),
            scope: Some("scope".to_string()),
            subject: Some("feat(scope): broadcast $destroy event on scope destruction".to_string()),
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_short_body() {
        let rule = BodyMaxLength {
            length: 10, // Short length for testing
            ..Default::default()
        };
        let message = Message {
            body: Some("Hello, I'm a long body".to_string()),
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction

Hello, I'm a long body"
                .to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            format!("body is longer than {} characters", rule.length)
        );
    }
}
