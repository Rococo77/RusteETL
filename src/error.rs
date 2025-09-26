//! Centralized ETL error types and conversions.
//!
//! Provides a simple error enum that converts from common error types used
//! across the crate.

#[derive(Debug)]
pub enum EtlError {
    /// IO errors (file read/write).
    Io(std::io::Error),
    /// CSV parsing/writing errors.
    Csv(csv::Error),
    /// HTTP client errors from reqwest
    Http(reqwest::Error),
    /// Placeholder for not implemented features.
    NotImplemented,
    /// Arbitrary other error with message.
    Other(String),
}

impl std::fmt::Display for EtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EtlError::Io(e) => write!(f, "IO error: {}", e),
            EtlError::Csv(e) => write!(f, "CSV error: {}", e),
            EtlError::Http(e) => write!(f, "HTTP error: {}", e),
            EtlError::NotImplemented => write!(f, "Not implemented"),
            EtlError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for EtlError {}

impl From<std::io::Error> for EtlError {
    fn from(e: std::io::Error) -> Self {
        EtlError::Io(e)
    }
}

impl From<csv::Error> for EtlError {
    fn from(e: csv::Error) -> Self {
        EtlError::Csv(e)
    }
}

impl From<calamine::Error> for EtlError {
    fn from(e: calamine::Error) -> Self {
        EtlError::Other(format!("Calamine error: {}", e))
    }
}

impl From<reqwest::Error> for EtlError {
    fn from(e: reqwest::Error) -> Self {
        EtlError::Http(e)
    }
}
