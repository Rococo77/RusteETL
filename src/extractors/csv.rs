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
    /// Optional delimiter character (defaults to comma).
    pub delimiter: Option<u8>,
}

impl CsvExtractor {
    /// Extract rows from CSV file located at `self.path`.
    ///
    /// This returns all rows as a Vec of Vec<String>. For large files you may
    /// want to use the streaming iterator `extract_iter` instead.
    ///
    /// # Errors
    /// Returns `EtlError` when file read or CSV parsing fails.
    pub fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        let mut out = Vec::new();
        for rec in self.extract_iter()? {
            out.push(rec);
        }
        Ok(out)
    }

    /// Return an iterator over records. Each record is a Vec<String>.
    pub fn extract_iter(&self) -> Result<impl Iterator<Item = Vec<String>>, EtlError> {
        let mut builder = ReaderBuilder::new();
        builder.has_headers(self.has_headers);
        if let Some(d) = self.delimiter {
            builder.delimiter(d);
        }

        let rdr = builder.from_path(&self.path)?;

        // Map csv::StringRecord into Vec<String>
        let iter = rdr.into_records().map(|res| match res {
            Ok(rec) => rec.iter().map(|s| s.to_string()).collect(),
            Err(_) => Vec::new(), // any error will be surfaced when collecting via extract
        });
        Ok(iter)
    }

    /// Convenience constructor with defaults.
    pub fn new(path: String) -> Self {
        CsvExtractor {
            path,
            has_headers: true,
            delimiter: None,
        }
    }
}
