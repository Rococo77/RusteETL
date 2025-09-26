use axum::{body::Bytes, http::StatusCode, routing::{get, post}, Router};
use axum::{response::IntoResponse, Json};

use crate::config::Config;

pub async fn health() -> StatusCode {
    StatusCode::OK
}

fn parse_pipeline_body(bytes: Bytes) -> Result<Config, String> {
    // Try JSON first
    if let Ok(cfg) = serde_json::from_slice::<Config>(&bytes) {
        return Ok(cfg);
    }
    // Fallback to YAML
    match serde_yaml::from_slice::<Config>(&bytes) {
        Ok(cfg) => Ok(cfg),
        Err(e) => Err(format!("Failed to parse payload as JSON or YAML: {}", e)),
    }
}
pub async fn post_pipeline(body: Bytes) -> impl IntoResponse {
    match parse_pipeline_body(body) {
        Ok(cfg) => match serde_json::to_value(cfg) {
            Ok(val) => (StatusCode::ACCEPTED, Json(val)),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("Failed to serialize parsed config: {}", e)}))),
        },
        Err(err) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": err}))),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/pipeline", post(post_pipeline))
}
