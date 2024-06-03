use spin_sdk::http::{IntoResponse, Params, Request, Response};
use crate::{enode_handlers::enode_http_get, models::{EnodeInverterResponse, EnodeInvertersResponse}};

pub(crate) async fn list_inverters(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Get all registered inverters...");

    let enode_uri = "/inverters";
    Ok(enode_http_get::<EnodeInvertersResponse>(enode_uri).await?)
}

pub(crate) async fn get_inverter(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("inverterId") else {
        return Ok(Response::new(404, "No inverter ID!!"))
    };
    println!("Fetch inverter: {}", id);

    let enode_uri = "/inverters/".to_string() + id;
    Ok(enode_http_get::<EnodeInverterResponse>(&enode_uri).await?)
}