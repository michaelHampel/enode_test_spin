use serde::{Deserialize, Serialize};
use super::Pagination;


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EnodeInvertersResponse {
    pub data: Vec<EnodeInverterResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct EnodeInverterResponse {
    pub id: String,
    pub userId: String,
    pub vendor: String,
    pub charginglocationId: Option<String>,
    pub lastSeen: String,
    pub isReachable: bool,
    pub productionState: InverterState,
    pub information: InverterInfo,
    pub location: InverterLocation,
    pub timezone: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct InverterState {
    pub productionRate: Option<f32>,
    pub isProducing: Option<bool>,
    pub totalLifetimeProduction: Option<f64>,
    pub lastUpdated: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct InverterInfo {
    pub id: String,
    pub brand: String,
    pub model: String,
    pub siteName: String,
    pub installationDate: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct InverterLocation {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
}