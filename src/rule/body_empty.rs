use crate::{message::Message, result::Violation, rule::Rule};
use serde::Deserialize;

use super::Level;

/// BodyEmpty represents the body-empty rule.
#[derive(Clone, Debug, Deserialize)]
pub struct BodyEmpty {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,
}

/// BodyEmpty represents the body-empty rule.
impl Rule for BodyEmpty {
    const NAME: &'static str = "body-empty";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        "body is empty".to_string()
    }

    fn validate(&self, message: &Message) -> Option<Violation> {
        if message.body.is_none() {
            return Some(Violation {
                level: self.level.unwrap_or(Self::LEVEL),
                message: self.message(message),
            });
        }

        None
    }
}

/// Default implementation of BodyEmpty.
impl Default for BodyEmpty {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
        }
    }
}
