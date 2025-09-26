//! CSV loader implementation.
//!
//! This loader writes rows to a CSV file at the configured `path`. If
//! `has_headers` is true and `headers` is provided, the header record will
//! be written before rows.

use crate::error::EtlError;
use csv::WriterBuilder;

/// CSV loader configuration and behaviour.
#[derive(Clone)]
pub struct CsvLoader {
    /// Path to the output CSV file.
    pub path: String,
    /// Whether to write headers.
    pub has_headers: bool,
    /// Optional headers to write when `has_headers` is true.
    pub headers: Option<Vec<String>>,
}

impl CsvLoader {
    /// Write rows to the configured CSV file.
    ///
    /// # Errors
    /// Returns `EtlError::Csv` or `EtlError::Io` when write/flush fails.
    pub fn load(&self, data: Vec<Vec<String>>) -> Result<(), EtlError> {
        let mut wtr = WriterBuilder::new()
            .has_headers(self.has_headers)
            .from_path(&self.path)?;

        if self.has_headers
            && let Some(ref headers) = self.headers
        {
            wtr.write_record(headers)?;
        }

        for row in data {
            wtr.write_record(&row)?;
        }

        wtr.flush()?;
        Ok(())
    }
}
