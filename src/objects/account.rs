use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub classic_address: String,
    pub address: Option<String>,
    pub xaddress: Option<String>,
    pub secret: Option<String>,
}
