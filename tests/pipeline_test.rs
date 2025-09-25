use ruste_etl::extractors::csv::CsvExtractor;
use ruste_etl::transformers::uppercase::UppercaseTransformer;
use ruste_etl::loaders::csv::CsvLoader;
use ruste_etl::pipeline::{Pipeline, Extractor, Transformer, Loader};

#[test]
fn test_pipeline_run() {
    let tmp_in = std::env::temp_dir().join(format!("ruste_etl_test_in_{}.csv", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
    let tmp_out = std::env::temp_dir().join(format!("ruste_etl_test_out_{}.csv", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
    std::fs::write(&tmp_in, "a,b\nhello,world\n").unwrap();

    let extractor = Extractor::Csv(CsvExtractor { path: tmp_in.to_string_lossy().to_string(), has_headers: true });
    let transformer = Transformer::Uppercase(UppercaseTransformer);
    let loader = Loader::Csv(CsvLoader { path: tmp_out.to_string_lossy().to_string(), has_headers: false, headers: None });
    let pipeline = Pipeline { extractor, transformer, loader };
    let result = pipeline.run();
    assert!(result.is_ok());

    let _ = std::fs::remove_file(&tmp_in);
    let _ = std::fs::remove_file(&tmp_out);
}
