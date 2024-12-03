use crate::{make_format_rule, message::Message, result::Violation, rule::Rule};

use super::Level;
make_format_rule! {
    ScopeFormat,
    "scope"
}

/// ScopeFormat represents the scope-format rule.
impl Rule for ScopeFormat {
    const NAME: &'static str = "scope-format";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!(
            "scope format does not match format: {}",
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

            match &message.scope {
                None => {
                    return Some(Violation {
                        level: self.level.unwrap_or(Self::LEVEL),
                        message: "found no scope".to_string(),
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

/// Default implementation of ScopeFormat.
impl Default for ScopeFormat {
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
        let rule = ScopeFormat {
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
        let rule = ScopeFormat {
            format: Some(r"^[a-z].*".to_string()),
            ..Default::default()
        };

        let message = Message {
            body: None,
            description: Some("Add new flag".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(Scope): Add new flag".to_string(),
            scope: Some("Scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            "scope format does not match format: ^[a-z].*".to_string()
        );
    }

    #[test]
    fn test_invalid_regex() {
        let rule = ScopeFormat {
            format: Some(r"(".to_string()),
            ..Default::default()
        };

        let message = Message {
            body: None,
            description: Some("Add regex".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): Add regex".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert!(violation.unwrap().message.contains("regex parse error"));
    }
}
