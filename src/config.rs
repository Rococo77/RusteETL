/// Module de configuration (YAML/JSON)
use serde::{Deserialize, Serialize};

/// Structure de configuration du pipeline ETL
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub extractor: ExtractorConfig,
    pub transformer: TransformerConfig,
    pub loader: LoaderConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractorConfig {
    pub kind: String, // "csv", "postgres", etc.
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformerConfig {
    pub kind: String, // "uppercase", "filter", etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderConfig {
    pub kind: String, // "csv", "postgres", etc.
    pub path: Option<String>,
}

/// Charge la configuration depuis un fichier YAML
pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
