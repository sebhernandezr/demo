use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DxWebviewRequest {
    pub url: String,
    pub label: String,
}
