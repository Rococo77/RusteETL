//! Binary entry point for the RusteETL CLI.
//!
//! This binary loads a YAML configuration and runs the pipeline described
//! by it. The configuration selects the extractor, transformer and loader.

use clap::Parser;
use ruste_etl::config::load_config;
use ruste_etl::error::EtlError;
use ruste_etl::extractors::xlsx::XlsxExtractor;
use ruste_etl::extractors::json::JsonExtractor;
use ruste_etl::extractors::{csv::CsvExtractor, postgres::PostgresExtractor};
use ruste_etl::loaders::{csv::CsvLoader, postgres::PostgresLoader, http::HttpLoader};
use ruste_etl::pipeline::{Extractor, Loader, Pipeline, Transformer};
use ruste_etl::transformers::{filter::FilterTransformer, map::{MapTransformer, MapOp}, uppercase::UppercaseTransformer};
use ruste_etl::utils::log_error;

#[derive(Parser)]
struct Cli {
    /// Path to the pipeline YAML config
    #[arg(short, long, default_value = "examples/pipeline.yml")]
    config: String,

    /// Dry-run: validate and parse config but do not execute pipeline
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Load configuration from YAML file
    let config = match load_config(&cli.config) {
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
            delimiter: config
                .extractor
                .delimiter
                .as_ref()
                .and_then(|s| s.as_bytes().first().cloned()),
        }),
        "xlsx" => {
            let path = config
                .extractor
                .path
                .clone()
                .unwrap_or_else(|| "input.xlsx".to_string());
            Extractor::Xlsx(XlsxExtractor {
                path,
                sheet: config.extractor.sheet.clone(),
            })
        }
        "postgres" => Extractor::Postgres(PostgresExtractor),
    "json" => Extractor::Json(JsonExtractor { path: config.extractor.path.clone().unwrap_or_else(|| "input.json".to_string()) }),
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
        "map" => {
            let col = config.transformer.column.unwrap_or(0);
            // parse the value as an operation: for simplicity support prefix:<v>, suffix:<v>, upper, lower, replace:<from>:<to>
            let value = config.transformer.value.clone().unwrap_or_default();
            let op = if value.starts_with("prefix:") {
                MapOp::Prefix(value[7..].to_string())
            } else if value.starts_with("suffix:") {
                MapOp::Suffix(value[7..].to_string())
            } else if value == "upper" {
                MapOp::Uppercase
            } else if value == "lower" {
                MapOp::Lowercase
            } else if value.starts_with("replace:") {
                let parts: Vec<&str> = value[8..].splitn(2, ':').collect();
                if parts.len() == 2 {
                    MapOp::Replace { from: parts[0].to_string(), to: parts[1].to_string() }
                } else {
                    MapOp::Prefix(value)
                }
            } else {
                MapOp::Prefix(value)
            };

            Transformer::Map(MapTransformer { column: col, operation: op })
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
        "http" => {
            let endpoint = config.loader.endpoint.clone().unwrap_or_else(|| "http://localhost:8080/".to_string());
            let retries = config.loader.retries.unwrap_or(3);
            let batch_size = config.loader.batch_size.unwrap_or(1000);
            let base_delay_ms = config.loader.base_delay_ms.unwrap_or(200);
            let jitter_ms = config.loader.jitter_ms.unwrap_or(100);
            Loader::Http(HttpLoader { endpoint, retries, batch_size, base_delay_ms, jitter_ms })
        }
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

    if cli.dry_run {
        println!("Dry-run OK: parsed and constructed pipeline successfully.");
        return;
    }

    if let Err(e) = pipeline.run().await {
        log_error(&e);
    }
}
