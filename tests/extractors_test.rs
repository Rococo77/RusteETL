use ruste_etl::extractors::CsvExtractor;

#[test]
fn test_csv_extractor() {
    // create a temporary CSV file
    let tmp = std::env::temp_dir().join(format!("ruste_etl_test_in_{}.csv", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
    let content = "col1,col2\nA,B\n";
    std::fs::write(&tmp, content).unwrap();

    let extractor = CsvExtractor { path: tmp.to_string_lossy().to_string(), has_headers: true };
    let data = extractor.extract().unwrap();
    assert_eq!(data[0][0], "A");

    let _ = std::fs::remove_file(&tmp);
}
