use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use ruste_etl::extractors::JsonExtractor;

fn write_temp(name: &str, content: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("ruste_etl_test_{}_{}.json", name, rand::random::<u32>()));
    let mut f = File::create(&path).expect("create temp file");
    f.write_all(content.as_bytes()).expect("write temp");
    path
}

#[test]
fn test_ndjson_extractor() {
    let content = r#"
["a","b","c"]
{"x":1, "y":"z"}
123
"#;
    let path = write_temp("ndjson", content);
    let extractor = JsonExtractor { path: path.to_string_lossy().to_string() };
    let rows = extractor.extract().expect("extract");
    assert!(!rows.is_empty());
}

#[test]
fn test_array_json_extractor() {
    let content = r#"[
  ["a","b"],
  ["c","d"]
]"#;
    let path = write_temp("array", content);
    let extractor = JsonExtractor { path: path.to_string_lossy().to_string() };
    let rows = extractor.extract().expect("extract");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["a".to_string(), "b".to_string()]);
}
