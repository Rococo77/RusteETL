//! CSV extractor implementation.
//!
//! Reads rows from a CSV file and returns them as a vector of string rows.

use crate::error::EtlError;
use csv::ReaderBuilder;

/// CSV extractor configuration.
pub struct CsvExtractor {
    /// Path to the input CSV file.
    pub path: String,
    /// Whether the CSV contains a header row.
    pub has_headers: bool,
}

impl CsvExtractor {
    /// Extract rows from CSV file located at `self.path`.
    ///
    /// # Errors
    /// Returns `EtlError` when file read or CSV parsing fails.
    pub fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(self.has_headers)
            .from_path(&self.path)?;

        let mut data = Vec::new();
        for record in rdr.records() {
            let rec = record?;
            data.push(rec.iter().map(|s| s.to_string()).collect());
        }
        Ok(data)
    }
}
