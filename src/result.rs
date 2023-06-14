/// Result of the check.
#[derive(Debug)]
pub struct Result {
    /// List of violations to be printed.
    /// If it is None, then there is no violation.
    pub violations: Option<Vec<String>>,
}
