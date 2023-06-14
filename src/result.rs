#[derive(Debug)]
pub struct Result {
    /// List of violations to be printed.
    pub violations: Option<Vec<String>>,
}
