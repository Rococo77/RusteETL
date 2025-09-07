use ruste_etl::loaders::{Loader, CsvLoader};

#[test]
fn test_csv_loader() {
    let loader = CsvLoader { path: "dummy_out.csv".to_string() };
    let data = vec![vec!["A".to_string(), "B".to_string()]];
    let result = loader.load(data);
    assert!(result.is_ok());
}
