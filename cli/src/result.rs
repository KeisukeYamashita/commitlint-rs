use crate::rule::Level;

/// Result of the check.
#[derive(Clone, Debug)]
pub struct Result {
    /// List of violations to be printed.
    /// If it is empty, then there is no violation.
    pub violations: Vec<Violation>,
}

/// Violation is a message that will be printed.
#[derive(Clone, Debug)]
pub struct Violation {
    /// Level of the violation.
    pub level: Level,

    /// Message of the violation.
    pub message: String,
}
