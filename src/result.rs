use crate::rule::Level;

/// Result of the check.
#[derive(Clone, Debug)]
pub struct Result {
    /// List of violations to be printed.
    /// If it is empty, then there is no violation.
    pub violations: Vec<Violation>,
}

#[derive(Clone, Debug)]
pub struct Violation {
    pub level: Level,
    pub message: String,
}
