
use serde::{Deserialize, Serialize};
use spin_sdk::http::{Method, Request, Response};

use crate::{enode_handlers::get_token, models::EnodeResponseError};

pub(crate) async fn enode_http_get<T>(uri: &str) -> anyhow::Result<Response> 
where
    T: for<'de> Deserialize<'de>,
    T: std::fmt::Debug,
    T: Serialize,
{
    let enode_url = std::env::var("API_URL").unwrap() + uri;

    let req = Request::builder()
        .uri(enode_url)
        .method(Method::Get)
        .build();

    handle_request_with_auth::<T>(req).await
} 

pub(crate) async fn enode_http_post<T>(uri: &str, body: String) -> anyhow::Result<Response> 
where
    T: for<'de> Deserialize<'de>,
    T: std::fmt::Debug,
    T: Serialize,
{
    let enode_url = std::env::var("API_URL").unwrap() + uri;

    let req = Request::builder()
        .uri(enode_url)
        .method(Method::Post)
        .header("Content-Type", "application/json")
        .body(body)
        .build();


    handle_request_with_auth::<T>(req).await
} 

pub(crate) async fn enode_http_put<T>(uri: &str, body: String) -> anyhow::Result<Response> 
where
    T: for<'de> Deserialize<'de>,
    T: std::fmt::Debug,
    T: Serialize,
{
    let enode_url = std::env::var("API_URL").unwrap() + uri;

    let req = Request::builder()
        .uri(enode_url)
        .method(Method::Put)
        .header("Content-Type", "application/json")
        .body(body)
        .build();


    handle_request_with_auth::<T>(req).await
} 

pub(crate) async fn enode_http_delete(uri: &str) -> anyhow::Result<Response> {
    let enode_url = std::env::var("API_URL").unwrap() + uri;

    let token: String = match check_token().await {
        Ok(t) => t,
        Err(e) => return Ok(e)
    };

    let req = Request::builder()
        .uri(enode_url)
        .method(Method::Delete)
        .header("Authorization", token)
        .build();

    Ok(spin_sdk::http::send(req).await?)
} 

pub(crate) async fn handle_request_with_auth<T>(mut req: Request) -> anyhow::Result<Response> 
where
    T: for<'de> Deserialize<'de>,
    T: std::fmt::Debug,
    T: Serialize,
{
    
    let token: String = match check_token().await {
        Ok(t) => t,
        Err(e) => return Ok(e)
    };

    req.set_header("Authorization", token);

    let resp: Response = spin_sdk::http::send(req).await?;

    match serde_json::from_slice::<T>(resp.body()) {
        Ok(body) => {
            println!("Got response from enode: {:#?}", body);
            Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&body)?))
        }
        Err(_) => {
            match serde_json::from_slice::<EnodeResponseError>(resp.body()) {
                Ok(e) => {
                    println!("Got error response from enode: {:#?}", e);
                    Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&e)?))
                }
                Err(_) => Ok(Response::new(resp.status().to_owned(), serde_json::to_string("Not able to handle enode response!!")?)),
            }
            
        }
    }

    /*match resp.status() {
        200..=208 => {
            let v: T = serde_json::from_slice(resp.body())?;
            println!("Got response from enode: {:#?}", v);
            Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&v)?))
        }
        400..=429 | 500..=511 => {
            let err_resp: EnodeResponseError = serde_json::from_slice(resp.body())?;
            println!("Got error response from enode: {:#?}", err_resp);
            return Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&err_resp)?))
        }
        _ => {
            println!("Not able to handle response from enode: {:#?}", resp);
            return Ok(Response::new(resp.status().to_owned(), serde_json::to_string("Not able to handle enode response!!")?))
        }
    }*/
}

pub(crate) async fn check_token() -> anyhow::Result<String, Response> {
    let Some(token) = get_token().await else {
        return Err(Response::new(401, "No valid token!!"))
    };

    println!("Token str: {}", token.header_str());
    Ok(token.header_str())
}

pub(crate) fn deserialize_from_bytes<T>(data: &[u8]) -> Result<T, serde_json::Error>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_slice(data)
}