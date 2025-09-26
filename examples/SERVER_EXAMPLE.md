Example server
===============

This repository includes an example Axum server in `src/bin/server.rs` as a
placeholder. To run a full async server, create a binary that initializes the
Tokio runtime and calls Axum's server binding. That server is intentionally
left out of automatic CI builds.

Example (for local use):

```rust
use std::net::SocketAddr;
use ruste_etl::api::router;
use axum::Router;

#[tokio::main]
async fn main() {
    let app: Router = router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

Place the runnable example in `src/bin/server.rs` or compile the example with a
feature flag that enables building the server binary.
