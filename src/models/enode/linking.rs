use serde::{Deserialize, Serialize};


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