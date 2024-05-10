use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use crate::{enode_handlers::get_token, models::{EnodeLinkRequest, EnodeLinkResponse, EnodeUser, EnodeUsers, EnodeVehiclesResponse, ResourceLinkRequest, ToEnodeLinkRequest}};

const SANDBOX_USER_NAME: &str = "miHam1";

/**
 * link a TESLA for user miHam1
 */
pub(crate) async fn link_sandbox_bev(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {

    let link_url = std::env::var("API_URL").unwrap() + "/users/" + SANDBOX_USER_NAME + "/link";
    println!("Link URL: {}", link_url);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
    };

    println!("Token str: {}", token.header_str());

    let link_body = EnodeLinkRequest {
        vendor: "TESLA".into(),
        vendorType: "vehicle".into(),
        language: "en-GB".into(),
        scopes: vec!["vehicle:read:data".into(), "vehicle:control:charging".into()],
        redirectUri: "myapp://integration/enode".into()
    };

    let json_body = serde_json::to_string(&link_body)?;
    println!("Send link body: {}", json_body);

    let link_req = Request::builder()
        .uri(link_url)
        .method(Method::Post)
        .header("Authorization", token.header_str())
        .header("Content-Type", "application/json")
        .body(json_body)
        .build();


    let link_resp: Response = spin_sdk::http::send(link_req).await?;
    println!("Link Response status: {}", link_resp.status());
  
    let enode_link_resp: EnodeLinkResponse = serde_json::from_slice(link_resp.body()).unwrap();
    println!("json Link Response body from enode: \n{:#?}", enode_link_resp);

    let serialized = serde_json::to_string(&enode_link_resp)?;
    Ok(Response::new(200, serialized))
    
}

pub(crate) async fn link_user_resource(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(link_data) = serde_json::from_slice::<ResourceLinkRequest>(req.body()) else {
        return Ok(Response::new(401, "Invalid data!!"))
    };
    let user_id = link_data.userId.as_str();
    println!("Link resource for user {} with data: {:#?}", user_id, link_data);


    let link_url = std::env::var("API_URL").unwrap() + "/users/" + user_id + "/link";
    println!("Link URL: {}", link_url);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
    };

    println!("Token str: {}", token.header_str());

    let link_body = link_data.to_enode();

    let json_body = serde_json::to_string(&link_body)?;
    println!("Send link body: {}", json_body);

    let link_req = Request::builder()
        .uri(link_url)
        .method(Method::Post)
        .header("Authorization", token.header_str())
        .header("Content-Type", "application/json")
        .body(json_body)
        .build();


    let link_resp: Response = spin_sdk::http::send(link_req).await?;
    println!("Link Response status: {}", link_resp.status());
  
    let enode_link_resp: EnodeLinkResponse = serde_json::from_slice(link_resp.body()).unwrap();
    println!("json Link Response body from enode: \n{:#?}", enode_link_resp);

    let serialized = serde_json::to_string(&enode_link_resp)?;
    Ok(Response::new(200, serialized))
    
}

pub(crate) async fn get_users(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Get all registered users...");

    let enode_url = std::env::var("API_URL").unwrap() + "/users";
    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
    };

    let get_req = Request::builder()
        .uri(enode_url)
        .method(Method::Get)
        .header("Authorization", token.header_str())
        .build();

    let resp: Response = spin_sdk::http::send(get_req).await?;
    let users: EnodeUsers = serde_json::from_slice(resp.body()).unwrap();

    println!("All registered users  from enode: {:#?}", users);

    Ok(Response::new(200, serde_json::to_string(&users)?))

}
/**
 * Get User Info
 */
pub(crate) async fn get_user(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(404, "No userID!!"))
    };
    println!("Fetch user info for: {}", user_id);

    let enode_url = std::env::var("API_URL").unwrap() + "/users/" + user_id;

    /*let rt = Runtime::new().unwrap();

    let Some(token) = rt.block_on(async { get_token().await}) else {
        return Ok(Response::new(401, String::new()))
    };*/

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
    };
    println!("Token str: {}", token.header_str());

    let user_req = Request::builder()
        .uri(enode_url)
        .method(Method::Get)
        .header("Authorization", token.header_str())
        .build();

    let user_resp: Response = spin_sdk::http::send(user_req).await?;
    let linked_user: EnodeUser = serde_json::from_slice(user_resp.body()).unwrap();

    println!("Got linked user from enode: {:#?}", linked_user);

    Ok(Response::new(200, serde_json::to_string(&linked_user)?))
}

/**
 * get linked vehcles for an user
 */
pub(crate) async fn get_user_vehicles(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(404, "No userID!!"))
    };
    println!("Fetch vehicles for user: {}", user_id);

    let enode_url = std::env::var("API_URL").unwrap() + "/users/" + user_id + "/vehicles";

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
    };
    println!("Token str: {}", token.header_str());

    let vehicles_req = Request::builder()
        .uri(enode_url)
        .method(Method::Get)
        .header("Authorization", token.header_str())
        .build();

    let vehicles_resp: Response = spin_sdk::http::send(vehicles_req).await?;
    println!("Vehicles response: {:#?}", vehicles_resp);
    let vehicles: EnodeVehiclesResponse = serde_json::from_slice(vehicles_resp.body()).unwrap();

    println!("Got vehicles for user {}: {:#?}", user_id, vehicles);

    Ok(Response::new(200, serde_json::to_string(&vehicles)?))
}

pub(crate) async fn unlink_user(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(401, "No userID!!"))
    };
    println!("Unlink user: {}", user_id);

    let enode_url = std::env::var("API_URL").unwrap() + "/users/" + user_id;

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
    };
    println!("Token str: {}", token.header_str());

    let user_req = Request::builder()
        .uri(enode_url)
        .method(Method::Delete)
        .header("Authorization", token.header_str())
        .build();

    let resp: Response = spin_sdk::http::send(user_req).await?;

    println!("Sucessfully unlinked user: {}", user_id);

    Ok(resp)
}