use crate::{make_format_rule, message::Message, result::Violation, rule::Rule};

use super::Level;
make_format_rule! {
    TypeFormat,
    "type"
}

/// TypeFormat represents the type-format rule.
impl Rule for TypeFormat {
    const NAME: &'static str = "type-format";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!(
            "type format does not match format: {}",
            self.format.as_ref().unwrap()
        )
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if let Some(format) = &self.format {
            let regex = match regex::Regex::new(format) {
                Ok(regex) => regex,
                Err(err) => {
                    return Some(Violation {
                        level: self.level.unwrap_or(Self::LEVEL),
                        message: err.to_string(),
                    });
                }
            };

            match &message.r#type {
                None => {
                    return Some(Violation {
                        level: self.level.unwrap_or(Self::LEVEL),
                        message: "found no type".to_string(),
                    });
                }
                Some(description) => {
                    if !regex.is_match(description) {
                        return Some(Violation {
                            level: self.level.unwrap_or(Self::LEVEL),
                            message: self.message(message),
                        });
                    }
                }
            }
        }

        None
    }
}

/// Default implementation of TypeFormat.
impl Default for TypeFormat {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
            format: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_description_format() {
        let rule = TypeFormat {
            format: Some(r"^[a-z].*".to_string()),
            ..Default::default()
        };

        let message = Message {
            body: None,
            description: Some("Add new flag".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): Add new flag".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_valid_description_format() {
        let rule = TypeFormat {
            format: Some(r"^[a-z].*".to_string()),
            ..Default::default()
        };

        let message = Message {
            body: None,
            description: Some("Add new flag".to_string()),
            footers: None,
            r#type: Some("Feat".to_string()),
            raw: "Feat(scope): Add new flag".to_string(),
            scope: Some("Scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            "type format does not match format: ^[a-z].*".to_string()
        );
    }

    #[test]
    fn test_invalid_regex() {
        let rule = TypeFormat {
            format: Some(r"(".to_string()),
            ..Default::default()
        };

        let message = Message {
            body: None,
            description: Some("Invalid regex".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): Invalid regex".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert!(violation.unwrap().message.contains("regex parse error"));
    }
}
