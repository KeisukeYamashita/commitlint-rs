use super::Level;
use crate::{make_options_rule, message::Message, result::Violation, rule::Rule};

make_options_rule! {
    Scope,
    "scope",
    #[doc = "Optional scope. If true, even if the scope is not present, it is allowed."]
    optional: bool
}

/// Scope represents the scope rule.
impl Rule for Scope {
    const NAME: &'static str = "scope";
    const LEVEL: Level = Level::Error;

    fn message(&self, message: &Message) -> String {
        if self.options.is_empty() {
            return "scopes are not allowed".to_string();
        }

        format!(
            "scope {} is not allowed. Only {:?} are allowed",
            message.scope.as_ref().unwrap_or(&"".to_string()),
            self.options
        )
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        match &message.scope {
            None => {
                if self.options.is_empty() || self.optional {
                    return None;
                }
            }
            Some(scope) if scope.is_empty() => {
                if self.options.is_empty() {
                    return None;
                }
            }
            Some(scope) if self.options.contains(scope) => {
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

/// Default implementation of Scope.
impl Default for Scope {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
            optional: false,
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
        fn test_empty_scope() {
            let rule = Scope::default();

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
        fn test_none_scope() {
            let rule = Scope::default();

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
        fn test_scope() {
            let rule = Scope::default();

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
                "scopes are not allowed".to_string()
            );
        }
    }

    mod scopes {
        use super::*;
        #[test]
        fn test_empty_scope() {
            let rule = Scope {
                options: vec!["api".to_string(), "web".to_string()],
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
                "scope  is not allowed. Only [\"api\", \"web\"] are allowed"
            );
        }

        #[test]
        fn test_none_scope() {
            let rule = Scope {
                options: vec!["api".to_string(), "web".to_string()],
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
                "scope  is not allowed. Only [\"api\", \"web\"] are allowed".to_string()
            );
        }

        #[test]
        fn test_valid_scope() {
            let rule = Scope {
                options: vec!["api".to_string(), "web".to_string()],
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
        fn test_invalid_scope() {
            let rule = Scope {
                options: vec!["api".to_string(), "web".to_string()],
                ..Default::default()
            };

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: Some("feat".to_string()),
                raw: "feat(invalid): broadcast $destroy event on scope destruction".to_string(),
                scope: Some("invalid".to_string()),
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_some());
            assert_eq!(violation.clone().unwrap().level, Level::Error);
            assert_eq!(
                violation.unwrap().message,
                "scope invalid is not allowed. Only [\"api\", \"web\"] are allowed".to_string()
            );
        }

        #[test]
        fn test_optional_scope_with_non_empty_scope() {
            let rule = Scope {
                options: vec!["api".to_string(), "web".to_string()],
                optional: true,
                ..Default::default()
            };

            let message = Message {
                body: None,
                description: None,
                footers: None,
                r#type: Some("feat".to_string()),
                raw: "feat(invalid): broadcast $destroy event on scope destruction".to_string(),
                scope: Some("invalid".to_string()),
                subject: None,
            };

            let violation = rule.validate(&message);
            assert!(violation.is_some());
            assert_eq!(violation.clone().unwrap().level, Level::Error);
            assert_eq!(
                violation.unwrap().message,
                "scope invalid is not allowed. Only [\"api\", \"web\"] are allowed".to_string()
            );
        }

        #[test]
        fn test_optional_scope_with_empty_scope() {
            let rule = Scope {
                options: vec!["api".to_string(), "web".to_string()],
                optional: true,
                ..Default::default()
            };

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
            assert!(violation.is_none());
        }
    }
}
