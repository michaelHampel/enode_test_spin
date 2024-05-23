use serde::{Deserialize, Serialize};

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
    pub failureReason: Option<ActionFailureReason>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ActionFailureReason {
    pub r#type: String,
    pub detail: String,
}