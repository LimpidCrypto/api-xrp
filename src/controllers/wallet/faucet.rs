#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{
    utils::networks::get_faucet_network_url, variants::network::Network,
    views::faucet::FaucetResponse,
};

use xrpl::wallet::Wallet;

use super::exceptions::WalletException;

pub async fn generate_faucet_wallet(
    Path(network): Path<Network>,
    State(_ctx): State<AppContext>,
) -> Result<Json<FaucetResponse>> {
    let wallet = match Wallet::create(None) {
        Ok(wallet) => wallet,
        Err(_) => return Err(Error::Anyhow(WalletException::FailedToCreateAccount.into())),
    };
    let x_address = wallet.get_xaddress(None, true).unwrap();
    let network_faucet_url = match get_faucet_network_url(network) {
        Ok(url) => url,
        Err(error) => return Err(Error::wrap(error)),
    };
    let mut funded_response = fund_wallet(network_faucet_url, wallet.classic_address.clone())
        .await
        .unwrap();
    if funded_response.account.xaddress.is_none() {
        funded_response.account.xaddress = Some(x_address.clone());
    }
    if funded_response.account.secret.is_none() {
        funded_response.account.secret = Some(wallet.seed.clone());
    }

    format::json(funded_response)
}

async fn fund_wallet(network_faucet_url: String, address: String) -> Result<FaucetResponse> {
    let client = reqwest::Client::new();
    let faucet_result = client
        .post(&network_faucet_url)
        .body(format!("{{\"destination\":\"{}\"}}", address))
        .send()
        .await;
    match faucet_result {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = response.text().await.unwrap();
                let response_json: FaucetResponse = serde_json::from_str(&response_text).unwrap();
                Ok(response_json)
            } else {
                Err(Error::Message(format!(
                    "Failed to fund wallet: {}",
                    response.status()
                )))
            }
        }
        Err(error) => Err(Error::wrap(error)),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("faucet")
        .add("/", get(generate_faucet_wallet))
}
