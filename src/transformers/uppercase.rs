//! Uppercase transformer.
//!
//! This transformer uppercases the first column of each row.

/// Uppercase transformer type.
pub struct UppercaseTransformer;

impl UppercaseTransformer {
    /// Apply uppercase transformation on the first column.
    pub fn transform(&self, mut data: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for row in &mut data {
            if let Some(first) = row.get_mut(0) {
                *first = first.to_uppercase();
            }
        }
        data
    }
}
