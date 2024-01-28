use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::account::Account;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Trace {
    pub hash: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct FaucetResponse {
    pub account: Account,
    pub amount: u32,
    pub balance: Option<u32>,
    pub trace: Option<Trace>,
}

impl FaucetResponse {
    #[must_use]
    pub fn new(account: &Account, amount: u32, balance: Option<u32>, trace: Option<Trace>) -> Self {
        Self {
            account: account.clone(),
            amount,
            balance,
            trace,
        }
    }
}
