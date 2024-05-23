use serde::{Deserialize, Serialize};


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
pub(crate) struct EnodeResponseError {
    pub r#type: String,
    pub title: String,
    pub detail: String,
    pub error: String,
    pub message: String,
}

