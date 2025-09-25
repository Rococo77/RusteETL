pub mod csv;
pub mod postgres;
pub mod xlsx;

// Re-export common types to preserve previous public API used by tests
pub use crate::pipeline::Extractor;
pub use csv::CsvExtractor;
pub use xlsx::XlsxExtractor;
