use serde::{Deserialize, Serialize};
use super::{Capabilities, Pagination};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EnodeVehiclesResponse {
    pub data: Vec<EnodeVehicleResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[allow(non_snake_case)]
pub(crate) struct EnodeVehicleResponse {
    pub id: String,
    pub userId: String,
    pub vendor: String,
    pub isReachable: bool,
    pub lastSeen: String,
    pub locationId: Option<String>,
    pub information: VehicleInfo,
    pub chargeState: VehicleChargeState,
    pub smartChargingPolicy: SmartChargingPolicy,
    pub location: VehicleLocation,
    pub odometer: Odometer,
    pub capabilities: Capabilities,
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub(crate) struct VehicleInfo {
    pub vin: String,
    pub brand: String,
    pub model: String,
    pub year: u32,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[allow(non_snake_case)]
pub(crate) struct VehicleChargeState {
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

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[allow(non_snake_case)]
pub(crate) struct SmartChargingPolicy {
    pub deadline: Option<String>,
    pub isEnabled: bool,
    pub minimumChargeLimit: u32,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[allow(non_snake_case)]
pub(crate) struct VehicleLocation {
    pub lastUpdated: Option<String>,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[allow(non_snake_case)]
pub(crate) struct Odometer {
    pub distance: u32,
    pub lastUpdated: String,
}