pub mod csv;
pub mod postgres;

// Re-export common types to preserve previous public API used by tests
pub use crate::pipeline::Extractor;
pub use csv::CsvExtractor;
