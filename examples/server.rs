fn main() {
    // Example placeholder to satisfy cargo when building examples during tests.
    // This binary is intentionally a no-op; real server belongs in src/bin when used locally.
    println!("Example server placeholder - no-op");
}
// Example server binary for local testing.
//
// The runnable example was moved here from src/bin to avoid building during CI.
// It is intentionally commented out to avoid compile-time issues across toolchains.
// To run the server locally, create a new binary (e.g. src/bin/server.rs) with
// the following contents, or enable a workspace feature that compiles it.
//
// ```rust
// use std::net::SocketAddr;
// use ruste_etl::api::router;
// use axum::Router;
//
// #[tokio::main]
// async fn main() {
//     let app: Router = router();
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Listening on http://{}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
// ```

// This file now only documents how to run the example server. If you want it
// compiled as part of the workspace, move it back to `src/bin/server.rs` and
// ensure the required dependencies and feature flags are present in Cargo.toml.
