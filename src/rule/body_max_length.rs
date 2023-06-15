use crate::{message::Message, result::Violation, rule::Rule};
use serde::Deserialize;

use super::Level;

/// BodyMaxLength represents the body-max-length rule.
#[derive(Clone, Debug, Deserialize)]
pub struct BodyMaxLength {
    /// Level represents the level of the rule.
    ///
    // Note that currently the default literal is not supported.
    // See: https://github.com/serde-rs/serde/issues/368
    level: Option<Level>,

    /// Length represents the maximum length of the body.
    length: usize,
}

/// BodyMaxLength represents the body-max-length rule.
impl Rule for BodyMaxLength {
    const NAME: &'static str = "body-max-length";
    const LEVEL: Level = Level::Error;

    fn message(&self, _message: &Message) -> String {
        format!("body is longer than {} characters", self.length)
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

/// Default implementation of BodyMaxLength.
impl Default for BodyMaxLength {
    fn default() -> Self {
        Self {
            level: Some(Self::LEVEL),
            length: 72,
        }
    }
}
