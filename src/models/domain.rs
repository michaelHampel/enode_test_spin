use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

use super::EnodeLinkRequest;

const DEFAULT_LANGUAGE: &str = "en-GB";
const DEFAULT_VEHICLE_SCOPES: Vec<String> =  vec!["vehicle:read:data".to_string(), "vehicle:control:charging".to_string()];
const ENODE_REDIRECT_URI: &str = "myapp://integration/enode";


#[derive(Debug, Deserialize, Serialize)]
pub(crate)struct EnodeClientToken {
    pub client: String,
    pub token: String,
    pub lifetime: i64
}

impl EnodeClientToken {
    pub fn header_str(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub fn is_valid(&self) -> bool {
        let dt = Utc.timestamp_opt(self.lifetime, 0).unwrap();
        Utc::now() < dt
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct ResourceLinkRequest {
    pub userId: String,
    pub vendor: String,
    pub vendorType: String,
}

pub(crate) trait ToEnodeLinkRequest {
    fn to_enode(self) -> EnodeLinkRequest;
}

impl ToEnodeLinkRequest for ResourceLinkRequest {
    fn to_enode(self) -> EnodeLinkRequest {
        EnodeLinkRequest {
            vendor: self.vendor,
            vendorType: self.vendorType,
            language: DEFAULT_LANGUAGE.to_string(),
            scopes: DEFAULT_VEHICLE_SCOPES,
            redirectUri: ENODE_REDIRECT_URI.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct UserRegistration {
    pub userId: String,
    pub first_name: String,
    pub last_name: String,
    pub pwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct User {
    pub userId: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct UserLogin {
    pub userId: String,
    pub pwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserTokenResponse {
    pub user_token: String,
}

pub(crate) struct DbSuccess{}