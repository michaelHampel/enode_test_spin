use spin_sdk::http::{IntoResponse, Params, Request, Response};

use crate::{enode_handlers::{enode_http_delete, enode_http_get, enode_http_put}, models::{UserLocation, UserLocationResponse, UserLocationsResponse}};



pub(crate) async fn list_locations(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Get all registered user locations...");

    let enode_uri = "/locations";
    Ok(enode_http_get::<UserLocationsResponse>(enode_uri).await?)
}

pub(crate) async fn get_location(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(location_id) = params.get("locationId") else {
        return Ok(Response::new(404, "No locationID!!"))
    };
    println!("Fetch location info for: {}", location_id);

    let enode_uri = "/locations/".to_string() + location_id;
    Ok(enode_http_get::<UserLocationResponse>(&enode_uri).await?)
}

pub(crate) async fn delete_location(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(location_id) = params.get("locationId") else {
        return Ok(Response::new(404, "No locationID!!"))
    };
    println!("Delete location: {}", location_id);

    let enode_uri = "/locations/".to_string() + location_id;
    Ok(enode_http_delete(&enode_uri).await?)
}

pub(crate) async fn update_location(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(location_data) = serde_json::from_slice::<UserLocation>(req.body()) else {
        return Ok(Response::new(401, "Invalid data!!"))
    };
    let Some(location_id) = params.get("locationId") else {
        return Ok(Response::new(404, "No locationID!!"))
    };
    println!("Update location: {:#?}", location_data);

    let enode_uri = "/locations/".to_string() + location_id;

    let json_body = serde_json::to_string(&location_data)?;
    println!("Update location with: {}", json_body);

    Ok(enode_http_put::<UserLocationResponse>(&enode_uri, json_body).await?)  
}