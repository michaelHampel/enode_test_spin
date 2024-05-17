use serde::{Deserialize, Serialize};
use spin_sdk::http::{Method, Request, Response};

use crate::enode_handlers::get_token;

pub(crate) async fn enode_http_get<T>(uri: &str) -> anyhow::Result<Response> 
where
    T: for<'de> Deserialize<'de>,
    T: std::fmt::Debug,
    T: Serialize,
{
    let enode_url = std::env::var("API_URL").unwrap() + uri;

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
    };
    println!("Token str: {}", token.header_str());

    let req = Request::builder()
        .uri(enode_url)
        .method(Method::Get)
        .header("Authorization", token.header_str())
        .build();

    let resp: Response = spin_sdk::http::send(req).await?;
    let v: T = serde_json::from_slice(resp.body()).unwrap();

    println!("Got response from enode: {:#?}", v);

    Ok(Response::new(200, serde_json::to_string(&v)?))
} 

pub(crate) fn deserialize_from_bytes<T>(data: &[u8]) -> Result<T, serde_json::Error>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_slice(data)
}