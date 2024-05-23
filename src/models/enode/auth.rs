use serde::Deserialize;


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