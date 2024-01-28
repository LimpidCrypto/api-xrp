use thiserror::Error;

use crate::variants::network::Network;

#[derive(Debug, Error)]
pub enum NetworkException {
    #[error("Unable to find network {0}. If your network is not listed, please open an issue at https://github.com/limpidcrypto/api-xrp/issues/new")]
    NoUrl(Network),
    #[error("Unable to find network {0}. If your network is not listed, please open an issue at https://github.com/limpidcrypto/api-xrp/issues/new")]
    NoUri(Network),
    #[error("Unable to find faucet url for network {0}. If your network is not listed, please open an issue at https://github.com/limpidcrypto/api-xrp/issues/new")]
    NoFaucetUrl(Network),
}
