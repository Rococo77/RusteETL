use ruste_etl::extractors::CsvExtractor;
use ruste_etl::transformers::UppercaseTransformer;
use ruste_etl::loaders::CsvLoader;
use ruste_etl::pipeline::Pipeline;

#[test]
fn test_pipeline_run() {
    let extractor = CsvExtractor { path: "input.csv".to_string() };
    let transformer = UppercaseTransformer;
    let loader = CsvLoader { path: "output.csv".to_string() };
    let pipeline = Pipeline {
        extractor: &extractor,
        transformer: &transformer,
        loader: &loader,
    };
    let result = pipeline.run();
    assert!(result.is_ok());
}
