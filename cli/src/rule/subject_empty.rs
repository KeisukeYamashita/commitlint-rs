use crate::{make_rule, message::Message, result::Violation, rule::Rule};

use super::Level;

make_rule! {
    SubjectEmpty,
}

/// SubjectEmpty represents the subject-empty rule.
impl Rule for SubjectEmpty {
    const NAME: &'static str = "subject-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "subject is empty".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.subject.is_none() {
            return Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            });
        }

        None
    }
}

/// Default implementation of SubjectEmpty.
impl Default for SubjectEmpty {
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
        let rule = SubjectEmpty::default();
        let message = Message {
            body: None,
            description: Some("broadcast $destroy event on scope destruction".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction

Hello world"
                .to_string(),
            scope: Some("scope".to_string()),
            subject: Some("feat(scope): broadcast $destroy event on scope destruction".to_string()),
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_empty_description() {
        let rule = SubjectEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "

Hello world"
                .to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(violation.unwrap().message, "subject is empty".to_string());
    }
}
