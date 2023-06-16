use crate::{message::Message, result::Violation, rule::Rule};
use serde::Deserialize;

use super::Level;

/// SubjectEmpty represents the subject-empty rule.
#[derive(Clone, Debug, Deserialize)]
pub struct SubjectEmpty {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,
}

/// SubjectEmpty represents the subject-empty rule.
impl Rule for SubjectEmpty {
    const NAME: &'static str = "subject-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "subject is empty or invalid".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.raw != "" {
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
