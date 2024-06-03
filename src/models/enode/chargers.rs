use serde::{Deserialize, Serialize};
use super::{Capability, Pagination};


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EnodeChargersResponse {
    pub data: Vec<EnodeChargerResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct EnodeChargerResponse {
    pub id: String,
    pub userId: String,
    pub vendor: String,
    pub isReachable: bool,
    pub locationId: Option<String>,
    pub chargeState: ChargerState,
    pub information: ChargerInfo,
    pub capabilities: ChargerCapabilities,
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct ChargerState {
    pub isPluggedIn: bool,
    pub isCharging: bool,
    pub chargeRate: Option<f32>,
    pub lastUpdated: String,
    pub powerDeliveryState: String,
    pub maxCurrent: u32,
}

#[derive(Serialize, Deserialize, Debug)]

pub(crate) struct ChargerInfo {
    pub brand: String,
    pub model: String,
    pub year: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct ChargerCapabilities {
    pub information: Capability,
    pub chargeState: Capability,
    pub setMaxCurrent: Capability,
    pub startCharging: Capability,
    pub stopCharging: Capability,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct ChargerLocation {
    pub locationId: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct ChargerMaxCurrent {
    pub maxCurrent: u16,
}