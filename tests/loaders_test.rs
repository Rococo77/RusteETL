use ruste_etl::loaders::CsvLoader;

#[test]
fn test_csv_loader() {
    let tmp = std::env::temp_dir().join(format!(
        "ruste_etl_test_out_{}.csv",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    ));
    let loader = CsvLoader {
        path: tmp.to_string_lossy().to_string(),
        has_headers: false,
        headers: None,
    };
    let data = vec![vec!["A".to_string(), "B".to_string()]];
    let result = loader.load(data);
    assert!(result.is_ok());
    let _ = std::fs::remove_file(&tmp);
}
