mod extractors;
mod transformers;
mod loaders;
mod pipeline;
mod config;
mod utils;

use extractors::CsvExtractor;
use transformers::UppercaseTransformer;
use loaders::CsvLoader;
use pipeline::Pipeline;
use utils::log_error;
use config::load_config;

fn main() {
    // Charge la configuration depuis un fichier YAML
    let config_path = "examples/pipeline.yml";
    let config = match load_config(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            log_error(e.as_ref());
            return;
        }
    };

    // Instancie les modules selon la config
    // Extractor
    let extractor: Box<dyn extractors::Extractor> = match config.extractor.kind.as_str() {
        "csv" => Box::new(CsvExtractor {
            path: config.extractor.path.clone().unwrap_or_else(|| "input.csv".to_string()),
        }),
        other => {
            eprintln!("Extracteur non supporté: {}", other);
            return;
        }
    };

    // Transformer
    let transformer: Box<dyn transformers::Transformer> = match config.transformer.kind.as_str() {
        "uppercase" => Box::new(UppercaseTransformer),
        other => {
            eprintln!("Transformateur non supporté: {}", other);
            return;
        }
    };

    // Loader
    let loader: Box<dyn loaders::Loader> = match config.loader.kind.as_str() {
        "csv" => Box::new(CsvLoader {
            path: config.loader.path.clone().unwrap_or_else(|| "output.csv".to_string()),
        }),
        other => {
            eprintln!("Loader non supporté: {}", other);
            return;
        }
    };

    let pipeline = Pipeline {
        extractor: extractor.as_ref(),
        transformer: transformer.as_ref(),
        loader: loader.as_ref(),
    };

    if let Err(e) = pipeline.run() {
        log_error(e.as_ref());
    }
}
