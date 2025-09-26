//! Configuration parsing and structures for the ETL pipeline.
//!
//! The configuration is deserialized from YAML and describes which
//! extractor/transformer/loader to use along with their options.

use serde::{Deserialize, Serialize};

/// Top-level pipeline configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Extractor configuration.
    pub extractor: ExtractorConfig,
    /// Transformer configuration.
    pub transformer: TransformerConfig,
    /// Loader configuration.
    pub loader: LoaderConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractorConfig {
    /// Kind of extractor ("csv", "postgres", ...).
    pub kind: String,
    /// Path to input file for file-based extractors.
    pub path: Option<String>,
    /// Whether CSV has headers.
    pub has_headers: Option<bool>,
    /// Optional delimiter for CSV (single character) represented as a string of length 1.
    pub delimiter: Option<String>,
    /// Optional sheet name for spreadsheet extractors.
    pub sheet: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformerConfig {
    /// Kind of transformer ("uppercase", "filter", ...).
    pub kind: String,
    /// Column to use for column-based transformers (zero-based).
    pub column: Option<usize>,
    /// Value used by certain transformers (e.g. filter).
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderConfig {
    /// Kind of loader ("csv", "postgres", ...).
    pub kind: String,
    /// Output path for file-based loaders.
    pub path: Option<String>,
    /// Whether to write headers for CSV loader.
    pub has_headers: Option<bool>,
    /// Optional headers to write.
    pub headers: Option<Vec<String>>,
    /// HTTP endpoint for http loader.
    pub endpoint: Option<String>,
    /// Number of retries for loaders that support retry/backoff.
    pub retries: Option<usize>,
    /// Number of rows per HTTP POST batch. If omitted, defaults to 1000.
    pub batch_size: Option<usize>,
    /// Base delay in milliseconds for retries (exponential backoff base). Optional.
    pub base_delay_ms: Option<u64>,
    /// Maximum jitter in milliseconds to add to backoff delay. Optional.
    pub jitter_ms: Option<u64>,
}

/// Load configuration from a YAML file at `path`.
pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
