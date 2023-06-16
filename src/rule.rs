use std::fmt::Debug;

use crate::{message::Message, result::Violation};
use serde::Deserialize;

use self::{
    body_empty::BodyEmpty, body_max_length::BodyMaxLength, description_empty::DescriptionEmpty,
    r#type::Type, scope::Scope, scope_empty::ScopeEmpty, subject_empty::SubjectEmpty,
    type_empty::TypeEmpty,
};

pub mod body_empty;
pub mod body_max_length;
pub mod description_empty;
pub mod scope;
pub mod scope_empty;
pub mod subject_empty;
pub mod r#type;
pub mod type_empty;

/// Rules represents the rules of commitlint.
/// See: https://commitlint.js.org/#/reference-rules
#[derive(Clone, Debug, Deserialize)]
pub struct Rules {
    #[serde(rename = "body-empty")]
    pub body_empty: Option<BodyEmpty>,

    #[serde(rename = "body-max-length")]
    pub body_max_length: Option<BodyMaxLength>,

    #[serde(rename = "description-empty")]
    pub description_empty: Option<DescriptionEmpty>,

    #[serde(rename = "scope")]
    pub scope: Option<Scope>,

    #[serde(rename = "scope-empty")]
    pub scope_empty: Option<ScopeEmpty>,

    #[serde(rename = "subject-empty")]
    pub subject_empty: Option<SubjectEmpty>,

    #[serde(rename = "type")]
    pub r#type: Option<Type>,

    #[serde(rename = "type-empty")]
    pub type_empty: Option<TypeEmpty>,
}

/// Rule is a collection of rules.
impl Rules {
    pub fn validate(&self, message: &Message) -> Vec<Violation> {
        let mut results = Vec::new();

        if let Some(rule) = &self.body_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.body_max_length {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.description_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.scope {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.scope_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.subject_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.r#type {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.type_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        results
    }
}

/// Default implementation of Rules.
/// If no config files are specified, this will be used.
impl Default for Rules {
    fn default() -> Self {
        Self {
            body_empty: None,
            body_max_length: None,
            description_empty: DescriptionEmpty::default().into(),
            scope: None,
            scope_empty: None,
            subject_empty: SubjectEmpty::default().into(),
            r#type: None,
            type_empty: TypeEmpty::default().into(),
        }
    }
}

/// Rule trait represents a rule that can be applied to a text.
pub trait Rule: Default {
    /// The name of the rule.
    /// Note that it should be unique
    const NAME: &'static str;

    /// The message to display when the rule fails.
    fn message(&self, message: &Message) -> String;

    /// The level of the rule.
    const LEVEL: Level;

    /// Validate the given text.
    fn validate(&self, message: &Message) -> Option<Violation>;
}

/// Level represents the level of a rule.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum Level {
    #[serde(rename = "error")]
    Error,

    #[serde(rename = "ignore")]
    Ignore,

    #[serde(rename = "warning")]
    Warning,
}
