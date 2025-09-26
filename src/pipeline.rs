//! Orchestration du pipeline ETL.
//!
//! This module provides `Extractor`, `Transformer`, `Loader` enums and the
//! high-level `Pipeline` which composes them. Each enum delegates to its
//! concrete implementation.

use crate::error::EtlError;
use crate::extractors::{csv::CsvExtractor, postgres::PostgresExtractor, xlsx::XlsxExtractor, json::JsonExtractor};
use crate::loaders::{csv::CsvLoader, postgres::PostgresLoader};
use crate::loaders::http::HttpLoader;
use crate::transformers::{filter::FilterTransformer, map::MapTransformer, uppercase::UppercaseTransformer};
use crate::retry;
use std::time::Duration;

/// Enum des extracteurs disponibles
/// Available extractor implementations.
pub enum Extractor {
    /// CSV extractor.
    Csv(CsvExtractor),
    /// XLSX extractor.
    Xlsx(XlsxExtractor),
    /// JSON extractor (NDJSON or JSON array)
    Json(JsonExtractor),
    /// PostgreSQL extractor (stub).
    Postgres(PostgresExtractor),
}

impl Extractor {
    /// Async extract dispatch. Uses blocking tasks for existing synchronous extractors.
    pub async fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        match self {
            Extractor::Csv(e) => e.extract_async().await,
            Extractor::Xlsx(e) => e.extract_async().await,
            Extractor::Json(e) => e.extract_async().await,
            Extractor::Postgres(e) => e.extract_async().await,
        }
    }
}

/// Enum des transformers disponibles
/// Available transformer implementations.
pub enum Transformer {
    /// Uppercase transformer.
    Uppercase(UppercaseTransformer),
    /// Filter transformer.
    Filter(FilterTransformer),
    /// Map transformer.
    Map(MapTransformer),
}

impl Transformer {
    pub fn transform(&self, data: Vec<Vec<String>>) -> Vec<Vec<String>> {
        match self {
            Transformer::Uppercase(t) => t.transform(data),
            Transformer::Filter(t) => t.transform(data),
            Transformer::Map(t) => t.transform(data),
        }
    }
}

/// Enum des loaders disponibles
/// Available loader implementations.
pub enum Loader {
    /// CSV loader.
    Csv(CsvLoader),
    /// PostgreSQL loader (stub).
    Postgres(PostgresLoader),
    /// HTTP loader (POST rows as JSON)
    Http(HttpLoader),
}

impl Loader {
    /// Async loader dispatch. Synchronous loaders run in a blocking task.
    pub async fn load(&self, data: Vec<Vec<String>>) -> Result<(), EtlError> {
        match self {
            Loader::Csv(l) => {
                let d = data;
                let l = l.clone();
                tokio::task::spawn_blocking(move || l.load(d))
                    .await
                    .map_err(|e| EtlError::Other(format!("Task join error: {}", e)))??;
                Ok(())
            }
            Loader::Postgres(l) => {
                let d = data;
                let l = l.clone();
                tokio::task::spawn_blocking(move || l.load(d))
                    .await
                    .map_err(|e| EtlError::Other(format!("Task join error: {}", e)))??;
                Ok(())
            }
            Loader::Http(l) => {
                let loader = l.clone();
                let d = data;
                let retries = loader.retries;
                let base_delay = Duration::from_millis(loader.base_delay_ms);
                let jitter = loader.jitter_ms;
                retry::retry_async(
                    || {
                        let loader = loader.clone();
                        let d = d.clone();
                        async move { loader.load(d).await }
                    },
                    retries,
                    base_delay,
                    jitter,
                )
                .await
                .map(|_| ())
            }
        }
    }
}

/// Pipeline ETL
/// High-level pipeline that composes an extractor, transformer and loader.
pub struct Pipeline {
    /// Extractor to use.
    pub extractor: Extractor,
    /// Transformer to apply.
    pub transformer: Transformer,
    /// Loader to persist results.
    pub loader: Loader,
}

impl Pipeline {
    /// Exécute le pipeline ETL complet
    pub async fn run(&self) -> Result<(), EtlError> {
        let data = self.extractor.extract().await?;
        let data = self.transformer.transform(data);
        self.loader.load(data).await?;
        Ok(())
    }
}
