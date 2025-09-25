//! Filter transformer.
//!
//! Keeps only rows where the specified column equals a configured value.

/// Filter transformer configuration.
pub struct FilterTransformer {
    /// Zero-based column index to examine.
    pub column: usize,
    /// Value to match for keeping a row.
    pub value: String,
}

impl FilterTransformer {
    /// Filter rows according to the configured column/value pair.
    pub fn transform(&self, data: Vec<Vec<String>>) -> Vec<Vec<String>> {
        data.into_iter()
            .filter(|row| row.get(self.column) == Some(&self.value))
            .collect()
    }
}
