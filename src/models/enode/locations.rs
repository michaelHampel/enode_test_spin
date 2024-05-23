use serde::{Deserialize, Serialize};
use super::Pagination;


#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct UserLocation {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub timezoneName: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct UserLocationResponse {
    pub id: String,
    pub userId: String,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub timezoneName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserLocationsResponse {
    pub data: Vec<UserLocationResponse>,
    pub pagination: Pagination,
}
