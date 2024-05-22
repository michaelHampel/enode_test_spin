
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::{enode_handlers::{enode_http_get, get_token}, models::{Action, ActionResponse, EnodeResponseError, EnodeVehicleResponse, EnodeVehiclesResponse}};

pub(crate) async fn get_vehicles(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Fetch all vehicle infos from enode...");

    let enode_uri = "/vehicles";
    Ok(enode_http_get::<EnodeVehiclesResponse>(&enode_uri).await?)
}

pub(crate) async fn get_vehicle(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(vehicle_id) = params.get("vehicleId") else {
        return Ok(Response::new(404, "No vehicleID provided!!"))
    };

    let enode_uri = "/vehicles/".to_string() + vehicle_id;
    Ok(enode_http_get::<EnodeVehicleResponse>(&enode_uri).await?)
}

pub(crate) async fn charge_vehicle(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(vehicle_id) = params.get("vehicleId") else {
        return Ok(Response::new(404, "No vehicleID!!"))
    };

    let enode_vehicle_url = std::env::var("API_URL").unwrap() + "/vehicles/" + vehicle_id + "/charging";

    let Ok(action) = serde_json::from_slice::<Action>(req.body()) else {
        return Ok(Response::new(401, "No valid token!!"))
    };

    println!("Charge action {} for vehicle {}", action.action, vehicle_id);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
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
    match resp.status() {
        200 => {
            let action_resp: ActionResponse = serde_json::from_slice(resp.body())?;
            println!("Got Action response for charging: {:#?}", action_resp);
            return Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&action_resp)?))
        }
        _ => {
            let error_resp: EnodeResponseError = serde_json::from_slice(resp.body())?;
            println!("Got Action response for charging: {:#?}", error_resp);
            return Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&error_resp)?))
        }
    }
    
}

/*pub(crate) async fn charge_vehicle_(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(vehicle_id) = params.get("vehicleId") else {
        return Ok(Response::new(404, "No vehicleID!!"))
    };

    let enode_uri = "/vehicles/".to_string() + vehicle_id + "/charging";

    let Ok(action) = serde_json::from_slice::<Action>(req.body()) else {
        return Ok(Response::new(401, "No valid token!!"))
    };

    println!("Charge action {} for vehicle {}", action.action, vehicle_id);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
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
    match resp.status() {
        200 => {
            let action_resp: ActionResponse = serde_json::from_slice(resp.body())?;
            println!("Got Action response for charging: {:#?}", action_resp);
            return Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&action_resp)?))
        }
        _ => {
            let error_resp: EnodeResponseError = serde_json::from_slice(resp.body())?;
            println!("Got Action response for charging: {:#?}", error_resp);
            return Ok(Response::new(resp.status().to_owned(), serde_json::to_string(&error_resp)?))
        }
    }
    
}*/

pub(crate) async fn get_vehicle_action(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(action_id) = params.get("actionId") else {
        return Ok(Response::new(404, "No actionID!!"))
    };

    let enode_url = std::env::var("API_URL").unwrap() + "/vehicles/actions/" + action_id;

    println!("Get vehicle action info for: {}", action_id);

    let Some(token) = get_token().await else {
        return Ok(Response::new(401, "No valid token!!"))
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
