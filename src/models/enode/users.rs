use serde::{Deserialize, Serialize};
use super::Pagination;
use utoipa::ToSchema;


#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub(crate) struct EnodeUser {
    pub id: String,
    pub linkedVendors: Vec<LinkedVendors>
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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