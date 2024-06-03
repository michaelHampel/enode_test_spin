
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use crate::{enode_handlers::{enode_http_delete, enode_http_get, enode_http_post, get_token}, models::{EnodeChargersResponse, EnodeInvertersResponse, EnodeLinkRequest, EnodeLinkResponse, EnodeUser, EnodeUsers, EnodeVehiclesResponse, ResourceLinkRequest, ToEnodeLinkRequest, UserLocation, UserLocationResponse, UserLocationsResponse}};

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


    let enode_uri = "/users/".to_string() + user_id + "/link";
    println!("Link URI: {}", enode_uri);

    let json_body = serde_json::to_string(&link_data.to_enode())?;
    println!("Send link body: {}", json_body);

    Ok(enode_http_post::<EnodeLinkResponse>(&enode_uri, json_body).await?)
    
}

pub(crate) async fn get_users(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Get all registered users...");

    let enode_uri = "/users";
    Ok(enode_http_get::<EnodeUsers>(enode_uri).await?)

}
/**
 * Get User Info
 */
#[utoipa::path(
    get,
    path = "/users/{userId}",
    responses(
        (status = 200, description = "User found successfully", body = EnodeUser),
        (status = NOT_FOUND, description = "User was not found")
    ),
    params(
        ("userId" = u64, Path, description = "UserId"),
    )
)]
pub(crate) async fn get_user(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(404, "No userID!!"))
    };
    println!("Fetch user info for: {}", user_id);

    let enode_uri = "/users/".to_string() + user_id;
    Ok(enode_http_get::<EnodeUser>(&enode_uri).await?)
}

/**
 * get linked vehcles for an user
 */
pub(crate) async fn list_user_vehicles(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(404, "No userID!!"))
    };
    println!("Fetch vehicles for user: {}", user_id);

    let enode_uri = "/users/".to_string() + user_id + "/vehicles";
    Ok(enode_http_get::<EnodeVehiclesResponse>(&enode_uri).await?)
}

pub(crate) async fn unlink_user(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(401, "No userID!!"))
    };
    println!("Unlink user: {}", user_id);

    let enode_uri = "/users/".to_string() + user_id;

    Ok(enode_http_delete(&enode_uri).await?)
}

pub(crate) async fn create_user_location(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(location_data) = serde_json::from_slice::<UserLocation>(req.body()) else {
        return Ok(Response::new(401, "Invalid data!!"))
    };
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(401, "No userID!!"))
    };
    println!("Create new location for user {} with data: {:#?}", user_id, location_data);


    let enode_uri = "/users/".to_string() + user_id + "/locations";

    let json_body = serde_json::to_string(&location_data)?;
    println!("Send location body: {}", json_body);

    Ok(enode_http_post::<UserLocationResponse>(&enode_uri, json_body).await?)  
}

pub(crate) async fn list_user_locations(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(404, "No userID!!"))
    };
    println!("Fetch locations for: {}", user_id);

    let enode_uri = "/users/".to_string() + user_id + "/locations";
    Ok(enode_http_get::<UserLocationsResponse>(&enode_uri).await?)
}

pub(crate) async fn list_user_chargers(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(404, "No userID!!"))
    };
    println!("Fetch chargers for: {}", user_id);

    let enode_uri = "/users/".to_string() + user_id + "/chargers";
    Ok(enode_http_get::<EnodeChargersResponse>(&enode_uri).await?)
}

pub(crate) async fn list_user_inverters(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(user_id) = params.get("userId") else {
        return Ok(Response::new(404, "No userID!!"))
    };
    println!("Fetch inverters for: {}", user_id);

    let enode_uri = "/users/".to_string() + user_id + "/inverters";
    Ok(enode_http_get::<EnodeInvertersResponse>(&enode_uri).await?)
}