use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EnodeTokenResponse {
    pub access_token: String,
    #[allow(unused)]
    pub expires_in: i32,
    #[allow(unused)]
    pub scope: Option<String>,
    #[allow(unused)]
    pub token_type: String
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct EnodeLinkRequest {
    pub vendor: String,
    pub vendorType: String,
    pub language: String,
    pub scopes: Vec<String>,
    pub redirectUri: String
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub(crate) struct EnodeLinkResponse {
    pub linkUrl: String,
    pub linkToken: String
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub(crate) struct EnodeUser {
    pub id: String,
    pub linkedVendors: Vec<LinkedVendors>
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub(crate) struct LinkedVendors {
    pub vendor: String,
    pub isValid: bool
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub(crate) struct EnodeUsers {
    pub data: Vec<UserInfo>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]

pub struct UserInfo {
    pub id: String,
    pub createdAt: String,
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EnodeVehiclesResponse {
    pub data: Vec<EnodeVehicleResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct EnodeVehicleResponse {
    pub id: String,
    pub userId: String,
    pub vendor: String,
    pub isReachable: bool,
    pub lastSeen: String,
    pub locationId: Option<String>,
    pub information: Information,
    pub chargeState: ChargeState,
    pub smartChargingPolicy: SmartChargingPolicy,
    pub location: Location,
    pub odometer: Odometer,
    pub capabilities: Capabilities,
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Information {
    pub vin: String,
    pub brand: String,
    pub model: String,
    pub year: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct ChargeState {
    pub chargeTimeRemaining: Option<u32>,
    pub chargeRate: Option<f32>,
    pub isFullyCharged: bool,
    pub isPluggedIn: bool,
    pub isCharging: bool,
    pub batteryLevel: u32,
    pub range: u32,
    pub batteryCapacity: u32,
    pub chargeLimit: u32,
    pub lastUpdated: String,
    pub powerDeliveryState: String,
    pub maxCurrent: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct SmartChargingPolicy {
    pub deadline: Option<String>,
    pub isEnabled: bool,
    pub minimumChargeLimit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct Location {
    pub lastUpdated: Option<String>,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct Odometer {
    pub distance: u32,
    pub lastUpdated: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct Capabilities {
    pub information: Capability,
    pub chargeState: Capability,
    pub location: Capability,
    pub odometer: Capability,
    pub setMaxCurrent: Capability,
    pub startCharging: Capability,
    pub stopCharging: Capability,
    pub smartCharging: Capability,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct Capability {
    pub interventionIds: Vec<String>,
    pub isCapable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Pagination {
    pub after: Option<String>,
    pub before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Action {
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct ActionResponse {
    pub id: String,
    pub userId: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub completedAt: Option<String>,
    pub state: String,
    pub targetId: String,
    pub targetType: String,
    pub kind: String,
    pub failureReason: Option<FailureReason>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FailureReason {
    pub r#type: String,
    pub detail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct EnodeResponseError {
    pub r#type: String,
    pub title: String,
    pub detail: String,
    pub error: String,
    pub message: String,
}