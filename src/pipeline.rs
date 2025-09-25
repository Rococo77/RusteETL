//! Orchestration du pipeline ETL.
//!
//! This module provides `Extractor`, `Transformer`, `Loader` enums and the
//! high-level `Pipeline` which composes them. Each enum delegates to its
//! concrete implementation.

use crate::error::EtlError;
use crate::extractors::{csv::CsvExtractor, postgres::PostgresExtractor, xlsx::XlsxExtractor};
use crate::loaders::{csv::CsvLoader, postgres::PostgresLoader};
use crate::transformers::{filter::FilterTransformer, uppercase::UppercaseTransformer};

/// Enum des extracteurs disponibles
/// Available extractor implementations.
pub enum Extractor {
    /// CSV extractor.
    Csv(CsvExtractor),
    /// XLSX extractor.
    Xlsx(XlsxExtractor),
    /// PostgreSQL extractor (stub).
    Postgres(PostgresExtractor),
}

impl Extractor {
    pub fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        match self {
            Extractor::Csv(e) => e.extract(),
            Extractor::Xlsx(e) => e.extract(),
            Extractor::Postgres(e) => e.extract(),
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
}

impl Transformer {
    pub fn transform(&self, data: Vec<Vec<String>>) -> Vec<Vec<String>> {
        match self {
            Transformer::Uppercase(t) => t.transform(data),
            Transformer::Filter(t) => t.transform(data),
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
}

impl Loader {
    pub fn load(&self, data: Vec<Vec<String>>) -> Result<(), EtlError> {
        match self {
            Loader::Csv(l) => l.load(data),
            Loader::Postgres(l) => l.load(data),
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
    pub fn run(&self) -> Result<(), EtlError> {
        let data = self.extractor.extract()?;
        let data = self.transformer.transform(data);
        self.loader.load(data)?;
        Ok(())
    }
}
