use anyhow::Ok;
use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Response};
use spin_contrib_http::cors::CorsResponseBuilder;

use crate::api::load_cors_config;

pub fn health(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let version = std::env::var("APP_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    let health = json!({"status": "ok", "version": version});

    Ok(Response::new(200, serde_json::to_string(&health)?)
        .into_builder()
        .build_with_cors(&req, &load_cors_config()))
}