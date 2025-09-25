//! Binary entry point for the RusteETL CLI.
//!
//! This binary loads a YAML configuration and runs the pipeline described
//! by it. The configuration selects the extractor, transformer and loader.

use ruste_etl::config::load_config;
use ruste_etl::error::EtlError;
use ruste_etl::extractors::{csv::CsvExtractor, postgres::PostgresExtractor};
use ruste_etl::loaders::{csv::CsvLoader, postgres::PostgresLoader};
use ruste_etl::pipeline::{Extractor, Loader, Pipeline, Transformer};
use ruste_etl::transformers::{filter::FilterTransformer, uppercase::UppercaseTransformer};
use ruste_etl::utils::log_error;

fn main() {
    // Load configuration from YAML file
    let config_path = "examples/pipeline.yml";
    let config = match load_config(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            log_error(e.as_ref());
            return;
        }
    };

    // Instantiate the extractor
    let extractor = match config.extractor.kind.as_str() {
        "csv" => Extractor::Csv(CsvExtractor {
            path: config
                .extractor
                .path
                .clone()
                .unwrap_or_else(|| "input.csv".to_string()),
            has_headers: config.extractor.has_headers.unwrap_or(true),
            delimiter: None,
        }),
        "postgres" => Extractor::Postgres(PostgresExtractor),
        other => {
            log_error(&EtlError::Other(format!(
                "Unsupported extractor: {}",
                other
            )));
            return;
        }
    };

    // Instantiate the transformer
    let transformer = match config.transformer.kind.as_str() {
        "uppercase" => Transformer::Uppercase(UppercaseTransformer),
        "filter" => {
            let col = config.transformer.column.unwrap_or(0);
            let val = config.transformer.value.clone().unwrap_or_default();
            Transformer::Filter(FilterTransformer {
                column: col,
                value: val,
            })
        }
        other => {
            log_error(&EtlError::Other(format!(
                "Unsupported transformer: {}",
                other
            )));
            return;
        }
    };

    // Instantiate the loader
    let loader = match config.loader.kind.as_str() {
        "csv" => Loader::Csv(CsvLoader {
            path: config
                .loader
                .path
                .clone()
                .unwrap_or_else(|| "output.csv".to_string()),
            has_headers: config.loader.has_headers.unwrap_or(true),
            headers: config.loader.headers.clone(),
        }),
        "postgres" => Loader::Postgres(PostgresLoader),
        other => {
            log_error(&EtlError::Other(format!("Unsupported loader: {}", other)));
            return;
        }
    };

    let pipeline = Pipeline {
        extractor,
        transformer,
        loader,
    };

    if let Err(e) = pipeline.run() {
        log_error(&e);
    }
}
