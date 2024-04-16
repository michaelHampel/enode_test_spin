use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use spin_sdk::http::{Method, Request, Response};

use crate::models::{EnodeClientToken, EnodeTokenResponse};
use crate::repository::{get_client_token, upsert_client_token};

const CLIENT_NAME: &str = "ENOX";

pub(crate) async fn get_token() -> Option<EnodeClientToken> {
    match get_client_token(CLIENT_NAME.to_string()).ok()? {
        None => {
            println!("Found no token for client in DB: {}, now get and save new token...", CLIENT_NAME);
            query_and_save_token().await
        }
        Some(t) if t.is_valid() => Some(t),
        Some(_) => {
            println!("Found expired token for client {} in DB, get and save new token...", CLIENT_NAME);
            query_and_save_token().await
        }
    }
}

async fn query_and_save_token() -> Option<EnodeClientToken> {
    let enode_token = query_token().await.ok()?;
    println!("got new token from enode: {:#?}", enode_token);
    let exp_timestamp = Utc::now().timestamp() + enode_token.expires_in as i64;
    match upsert_client_token(CLIENT_NAME.to_string(), enode_token.access_token, exp_timestamp) {
        Err(e) => {
            eprintln!("Error upserting token: {}", e.to_string());
            None
        }
        Ok(t) => Some(t)
    }    
}

async fn query_token() -> anyhow::Result<EnodeTokenResponse> {
    let o_auth_url = std::env::var("OAUTH_URL").unwrap();
    println!("Env OAuth_Urln loaded: {}", o_auth_url);


    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();

    let auth_basic = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(format!(
            "{}:{}",
            client_id,
            client_secret
        ))
    );

    println!("Basic_auth string: {}", auth_basic);

    let token_request = Request::builder()
     .method(Method::Post)
     .uri(o_auth_url)
     .body("grant_type=client_credentials")
     .header("Content-Type","application/x-www-form-urlencoded")
     .header("Authorization", auth_basic)
     .build();

    let resp: Response = spin_sdk::http::send(token_request).await?;
    
    println!("Http token resp Status:: {}", resp.status());

    /*let body_str = String::from_utf8(resp.body().to_vec());
    println!("Response body from enode oAuth: \n{:#?}", body_str);*/

    let token_resp: EnodeTokenResponse = serde_json::from_slice(resp.body()).unwrap();
    println!("json Response body from enode oAuth: \n{:#?}", token_resp);

    Ok(token_resp)
}