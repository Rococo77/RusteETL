use ruste_etl::extractors::{Extractor, CsvExtractor};

#[test]
fn test_csv_extractor() {
    let extractor = CsvExtractor { path: "dummy.csv".to_string() };
    let data = extractor.extract().unwrap();
    assert_eq!(data, vec![vec!["col1".to_string(), "col2".to_string()]]);
}
