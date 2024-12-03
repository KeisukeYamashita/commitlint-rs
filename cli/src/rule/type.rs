use crate::{make_options_rule, message::Message, result::Violation, rule::Rule};

use super::Level;
make_options_rule! {
    Type,
    "type",
}

/// Type represents the type rule.
impl Rule for Type {
    const NAME: &'static str = "type";
    const LEVEL: Level = Level::Error;
    fn message(&self, message: &Message) -> String {
        if self.options.is_empty() {
            return "types are not allowed".to_string();
        }

        format!(
            "type {} is not allowed. Only {:?} are allowed",
            message.r#type.as_ref().unwrap_or(&"".to_string()),
            self.options
        )
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        match &message.r#type {
            None => {
                if self.options.is_empty() {
                    return None;
                }
            }
            Some(r#type) if r#type.is_empty() => {
                if self.options.is_empty() {
                    return None;
                }
            }
            Some(r#type) if self.options.contains(r#type) => {
                return None;
            }
            _ => {}
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

    mod empty_options {
        use super::*;

        #[test]
        fn test_empty_type() {
            let rule = Type::default();

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: None,
                raw: "".to_string(),
                scope: Some("".to_string()),
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_none());
        }

        #[test]
        fn test_none_type() {
            let rule = Type::default();

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: None,
                raw: "".to_string(),
                scope: None,
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_none());
        }

        #[test]
        fn test_type() {
            let rule = Type::default();

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: Some("feat".to_string()),
                raw: "feat(web): broadcast $destroy event on scope destruction".to_string(),
                scope: Some("web".to_string()),
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_some());
            assert_eq!(violation.clone().unwrap().level, Level::Error);
            assert_eq!(
                violation.unwrap().message,
                "types are not allowed".to_string()
            );
        }
    }

    mod scopes {
        use super::*;
        #[test]
        fn test_empty_type() {
            let rule = Type {
                options: vec!["feat".to_string(), "chore".to_string()],
                ..Default::default()
            };

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: None,
                raw: "".to_string(),
                scope: Some("".to_string()),
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_some());
            assert_eq!(violation.clone().unwrap().level, Level::Error);
            assert_eq!(
                violation.unwrap().message,
                "type  is not allowed. Only [\"feat\", \"chore\"] are allowed"
            );
        }

        #[test]
        fn test_none_type() {
            let rule = Type {
                options: vec!["feat".to_string(), "chore".to_string()],
                ..Default::default()
            };

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: None,
                raw: "".to_string(),
                scope: None,
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_some());
            assert_eq!(violation.clone().unwrap().level, Level::Error);
            assert_eq!(
                violation.unwrap().message,
                "type  is not allowed. Only [\"feat\", \"chore\"] are allowed".to_string()
            );
        }

        #[test]
        fn test_valid_type() {
            let rule = Type {
                options: vec!["feat".to_string(), "chore".to_string()],
                ..Default::default()
            };

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: Some("feat".to_string()),
                raw: "feat(web): broadcast $destroy event on scope destruction".to_string(),
                scope: Some("web".to_string()),
                subject: None,
            };

            assert!(rule.validate(&message).is_none());
        }

        #[test]
        fn test_invalid_type() {
            let rule = Type {
                options: vec!["feat".to_string(), "chore".to_string()],
                ..Default::default()
            };

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: Some("invalid".to_string()),
                raw: "invalid(web): broadcast $destroy event on scope destruction".to_string(),
                scope: Some("web".to_string()),
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_some());
            assert_eq!(violation.clone().unwrap().level, Level::Error);
            assert_eq!(
                violation.unwrap().message,
                "type invalid is not allowed. Only [\"feat\", \"chore\"] are allowed".to_string()
            );
        }
    }
}
