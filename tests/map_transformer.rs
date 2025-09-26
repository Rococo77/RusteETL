use ruste_etl::transformers::map::{MapOp, MapTransformer};

#[test]
fn test_map_prefix() {
    let data = vec![vec!["foo".to_string(), "bar".to_string()]];
    let t = MapTransformer { column: 0, operation: MapOp::Prefix("pre-".to_string()) };
    let out = t.transform(data);
    assert_eq!(out[0][0], "pre-foo");
}

#[test]
fn test_map_replace() {
    let data = vec![vec!["a-b-c".to_string()]];
    let t = MapTransformer { column: 0, operation: MapOp::Replace { from: "-b".to_string(), to: "".to_string() } };
    let out = t.transform(data);
    assert_eq!(out[0][0], "a-c");
}

#[test]
fn test_map_suffix() {
    let data = vec![vec!["foo".to_string()]];
    let t = MapTransformer { column: 0, operation: MapOp::Suffix("-suf".to_string()) };
    let out = t.transform(data);
    assert_eq!(out[0][0], "foo-suf");
}

#[test]
fn test_map_uppercase() {
    let data = vec![vec!["Abc".to_string()]];
    let t = MapTransformer { column: 0, operation: MapOp::Uppercase };
    let out = t.transform(data);
    assert_eq!(out[0][0], "ABC");
}

#[test]
fn test_map_lowercase() {
    let data = vec![vec!["XYz".to_string()]];
    let t = MapTransformer { column: 0, operation: MapOp::Lowercase };
    let out = t.transform(data);
    assert_eq!(out[0][0], "xyz");
}

#[test]
fn test_map_out_of_bounds_column() {
    // If the specified column index is out of bounds, transformer should leave row unchanged
    let data = vec![vec!["only".to_string()]];
    let t = MapTransformer { column: 5, operation: MapOp::Prefix("p-".to_string()) };
    let out = t.transform(data);
    assert_eq!(out[0][0], "only");
}
