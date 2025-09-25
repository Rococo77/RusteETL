use ruste_etl::transformers::uppercase::UppercaseTransformer;

#[test]
fn test_uppercase_transformer() {
    let transformer = UppercaseTransformer;
    let input = vec![vec!["abc".to_string(), "def".to_string()]];
    let output = transformer.transform(input);
    assert_eq!(output[0][0], "ABC");
}
