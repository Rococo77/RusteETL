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
}

/// Load configuration from a YAML file at `path`.
pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
