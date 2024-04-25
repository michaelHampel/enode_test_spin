use anyhow::Ok;
use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Response};

pub fn health(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let version = std::env::var("APP_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    let health = json!({"status": "ok", "version": version});

    Ok(Response::new(200, serde_json::to_string(&health)?))
}