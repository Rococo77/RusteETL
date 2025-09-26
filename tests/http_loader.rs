use httpmock::MockServer;
use httpmock::Method::POST;
use ruste_etl::loaders::HttpLoader;

#[tokio::test]
async fn test_http_loader_posts_rows() {
    // start a local mock server
    let server = MockServer::start();

    // expect a single POST request with batch payload
    let mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200);
    });

    let loader = HttpLoader { endpoint: server.url("/"), retries: 1, batch_size: 10, base_delay_ms: 10, jitter_ms: 5 };

    let data = vec![vec!["one".to_string()], vec!["two".to_string()]];
    let res = loader.load(data).await;
    assert!(res.is_ok());

    // assert the server received exactly one request (batch)
    mock.assert();
    assert_eq!(mock.hits(), 1);
}
