pub mod csv;
pub mod postgres;
pub mod http;

// Re-export common types for backward compatibility
pub use crate::pipeline::Loader;
pub use csv::CsvLoader;
pub use http::HttpLoader;
