pub mod csv;
pub mod postgres;

// Re-export common types for backward compatibility
pub use crate::pipeline::Loader;
pub use csv::CsvLoader;
