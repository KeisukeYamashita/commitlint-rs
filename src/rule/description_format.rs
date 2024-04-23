use crate::{message::Message, result::Violation, rule::Rule};
use serde::{Deserialize, Serialize};

use super::Level;

/// DescriptionFormat represents the description-format rule.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DescriptionFormat {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,

    /// Format represents the format of the description.
    format: Option<String>,
}

/// DescriptionFormat represents the description-format rule.
impl Rule for DescriptionFormat {
    const NAME: &'static str = "description-format";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!(
            "description format does not match format: {}",
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

            if !regex.is_match(&message.description.as_ref().unwrap()) {
                return Some(Violation {
                    level: self.level.unwrap_or(Self::LEVEL),
                    message: self.message(message),
                });
            }
        }

        None
    }
}

/// Default implementation of DescriptionFormat.
impl Default for DescriptionFormat {
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
        let mut rule = DescriptionFormat::default();
        rule.format = Some(r"^[a-z].*".to_string());

        let message = Message {
            body: None,
            description: Some("add new flag".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): add new flag".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_valid_description_format() {
        let mut rule = DescriptionFormat::default();
        rule.format = Some(r"^[a-z].*".to_string());

        let message = Message {
            body: None,
            description: Some("Add new flag".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): Add new flag".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(
            violation.unwrap().message,
            "description format does not match format: ^[a-z].*".to_string()
        );
    }

    #[test]
    fn test_invalid_regex() {
        let mut rule = DescriptionFormat::default();
        rule.format = Some(r"(".to_string());

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
