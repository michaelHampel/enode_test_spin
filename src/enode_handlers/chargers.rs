use spin_sdk::http::{IntoResponse, Params, Request, Response};

use crate::{enode_handlers::{enode_http_get, enode_http_post, enode_http_put}, models::{Action, ActionResponse, ChargerLocation, ChargerMaxCurrent, EnodeChargerResponse, EnodeChargersResponse}};

pub(crate) async fn list_chargers(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Get all registered chargers...");

    let enode_uri = "/chargers";
    Ok(enode_http_get::<EnodeChargersResponse>(enode_uri).await?)
}

pub(crate) async fn get_charger(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("chargerId") else {
        return Ok(Response::new(404, "No charger ID!!"))
    };
    println!("Fetch charger: {}", id);

    let enode_uri = "/locations/".to_string() + id;
    Ok(enode_http_get::<EnodeChargerResponse>(&enode_uri).await?)
}

pub(crate) async fn update_charger(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(location_data) = serde_json::from_slice::<ChargerLocation>(req.body()) else {
        return Ok(Response::new(401, "Invalid data!!"))
    };
    let Some(id) = params.get("chargerId") else {
        return Ok(Response::new(404, "No charger ID!!"))
    };
    println!("Update charger location: {:#?}", location_data);

    let enode_uri = "/chargers/".to_string() + id;

    let json_body = serde_json::to_string(&location_data)?;
    println!("Update charger location with: {}", json_body);

    Ok(enode_http_put::<EnodeChargerResponse>(&enode_uri, json_body).await?)  
}

pub(crate) async fn set_max_current(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(data) = serde_json::from_slice::<ChargerMaxCurrent>(req.body()) else {
        return Ok(Response::new(401, "Invalid data!!"))
    };
    let Some(id) = params.get("chargerId") else {
        return Ok(Response::new(404, "No charger ID!!"))
    };
    println!("Update charger max current: {:#?}", data);

    let enode_uri = "/chargers/".to_string() + id + "/max-current";

    let json_body = serde_json::to_string(&data)?;
    println!("Update charger max_current with: {}", json_body);

    Ok(enode_http_put::<ActionResponse>(&enode_uri, json_body).await?)  
}

pub(crate) async fn control_charger(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("chargerId") else {
        return Ok(Response::new(404, "No charger ID!!"))
    };

    let enode_uri = "/chargers/".to_string() + id + "/charging";

    let Ok(action) = serde_json::from_slice::<Action>(req.body()) else {
        return Ok(Response::new(404, "Malformed request body!!"))
    };

    println!("Charge action {} for charger {}", action.action, id);

    let json_body = serde_json::to_string(&action)?;
    println!("Send action body: {}", json_body);

    Ok(enode_http_post::<ActionResponse>(&enode_uri, json_body).await?)
    
}

pub(crate) async fn get_charger_action(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(action_id) = params.get("actionId") else {
        return Ok(Response::new(404, "No actionID!!"))
    };
    println!("Get charger action info for action_id: {}", action_id);
    let enode_uri = "/chargers/actions/".to_string() + action_id;

    Ok(enode_http_get::<ActionResponse>(&enode_uri).await?)
}

pub(crate) async fn cancel_charger_action(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("chargerId") else {
        return Ok(Response::new(404, "No charger ID!!"))
    };

    let enode_uri = "/chargers/actions/".to_string() + id + "/cancel";

    let Ok(action) = serde_json::from_slice::<Action>(req.body()) else {
        return Ok(Response::new(404, "Malformed request body!!"))
    };

    println!("Charge action {} for charger {}", action.action, id);

    let json_body = serde_json::to_string(&action)?;
    println!("Send action body: {}", json_body);

    Ok(enode_http_post::<ActionResponse>(&enode_uri, json_body).await?)
    
}