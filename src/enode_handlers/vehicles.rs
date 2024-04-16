use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::{enode_handlers::get_token, models::{Action, ActionResponse, EnodeVehicleResponse, EnodeVehiclesResponse}};

pub(crate) async fn get_vehicles(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let enode_vehicles_url = std::env::var("API_URL").unwrap() + "/vehicles";

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, String::new()))
    };
    println!("Token str: {}", token.header_str());

    let vehicles_req = Request::builder()
        .uri(enode_vehicles_url)
        .method(Method::Get)
        .header("Authorization", token.header_str())
        .build();

    let vehicles_resp: Response = spin_sdk::http::send(vehicles_req).await?;
    let vehicles: EnodeVehiclesResponse = serde_json::from_slice(vehicles_resp.body()).unwrap();

    println!("Got vehicles from enode: {:#?}", vehicles);

    Ok(Response::new(200, serde_json::to_string(&vehicles)?))
}


pub(crate) async fn get_vehicle(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(vehicle_id) = params.get("vehicleId") else {
        return Ok(Response::new(404, String::new()))
    };

    let enode_vehicle_url = std::env::var("API_URL").unwrap() + "/vehicles/" + vehicle_id;

    println!("Fetch vehicle info for: {}", vehicle_id);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, String::new()))
    };
    println!("Token str: {}", token.header_str());

    let user_req = Request::builder()
        .uri(enode_vehicle_url)
        .method(Method::Get)
        .header("Authorization", token.header_str())
        .build();

    let vehicle_resp: Response = spin_sdk::http::send(user_req).await?;
    let vehicle: EnodeVehicleResponse = serde_json::from_slice(vehicle_resp.body()).unwrap();

    println!("Got linked user from enode: {:#?}", vehicle);

    Ok(Response::new(200, serde_json::to_string(&vehicle)?))
}

pub(crate) async fn charge_vehicle(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(vehicle_id) = params.get("vehicleId") else {
        return Ok(Response::new(404, String::new()))
    };

    let enode_vehicle_url = std::env::var("API_URL").unwrap() + "/vehicles/" + vehicle_id + "/charging";

    let Ok(action) = serde_json::from_slice::<Action>(req.body()) else {
        return Ok(Response::new(400, ()))
    };

    println!("Charge action {} for vehicle {}", action.action, vehicle_id);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, String::new()))
    };

    let json_body = serde_json::to_string(&action)?;
    println!("Send action body: {}", json_body);

    let action_req = Request::builder()
        .uri(enode_vehicle_url)
        .method(Method::Post)
        .header("Authorization", token.header_str())
        .header("Content-Type", "application/json")
        .body(json_body)
        .build();

    let resp: Response = spin_sdk::http::send(action_req).await?;
    let action_resp: ActionResponse = serde_json::from_slice(resp.body()).unwrap();

    println!("Got Action response for charging: {:#?}", action_resp);

    Ok(Response::new(200, serde_json::to_string(&action_resp)?))
}

pub(crate) async fn get_vehicle_action(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(action_id) = params.get("actionId") else {
        return Ok(Response::new(404, String::new()))
    };

    let enode_url = std::env::var("API_URL").unwrap() + "/vehicles/actions/" + action_id;

    println!("Get vehicle action info for: {}", action_id);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, String::new()))
    };

    let action_req = Request::builder()
        .uri(enode_url)
        .method(Method::Get)
        .header("Authorization", token.header_str())
        .build();

    let resp: Response = spin_sdk::http::send(action_req).await?;
    let action: ActionResponse = serde_json::from_slice(resp.body()).unwrap();

    println!("Got vehicle action: {:#?}", action);

    Ok(Response::new(200, serde_json::to_string(&action)?))
}
