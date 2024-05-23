
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::{enode_handlers::{enode_http_get, enode_http_post, get_token}, models::{Action, ActionResponse, EnodeResponseError, EnodeVehicleResponse, EnodeVehiclesResponse}};

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

    let enode_uri = "/vehicles/".to_string() + vehicle_id + "/charging";

    let Ok(action) = serde_json::from_slice::<Action>(req.body()) else {
        return Ok(Response::new(404, "Malformed request body!!"))
    };

    println!("Charge action {} for vehicle {}", action.action, vehicle_id);

    let json_body = serde_json::to_string(&action)?;
    println!("Send action body: {}", json_body);

    Ok(enode_http_post::<ActionResponse>(&enode_uri, json_body).await?)
    
}

pub(crate) async fn get_vehicle_action(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(action_id) = params.get("actionId") else {
        return Ok(Response::new(404, "No actionID!!"))
    };
    println!("Get vehicle action info for action_id: {}", action_id);
    let enode_uri = "/vehicles/actions/".to_string() + action_id;

    Ok(enode_http_get::<ActionResponse>(&enode_uri).await?)
}
