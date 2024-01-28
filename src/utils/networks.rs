use crate::variants::network::Network;

use anyhow::Result;

use super::exceptions::NetworkException;

pub fn get_network_url(network: Network) -> Result<String, NetworkException> {
    match network {
        Network::Test => Ok("https://testnet.xrpl-labs.com/".to_string()),
        Network::Dev => Ok("https://s.devnet.rippletest.net:51234/".to_string()),
        Network::XahauTest => Ok("https://xahau-test.net/".to_string()),
        Network::SidechainTest => {
            Ok("https://sidechain-net2.devnet.rippletest.net:51234/".to_string())
        }
        Network::Custom(url) => Ok(url),
    }
}

pub fn get_network_uri(network: Network) -> Result<String, NetworkException> {
    match network {
        Network::Test => Ok("wss://testnet.xrpl-labs.com/".to_string()),
        Network::Dev => Ok("wss://s.devnet.rippletest.net:51233/".to_string()),
        Network::XahauTest => Ok("wss://xahau-test.net/".to_string()),
        Network::SidechainTest => {
            Ok("wss://sidechain-net2.devnet.rippletest.net:51233/".to_string())
        }
        Network::Custom(url) => Ok(url),
    }
}

pub fn get_faucet_network_url(network: Network) -> Result<String, NetworkException> {
    match network {
        Network::Test => Ok("https://faucet.altnet.rippletest.net/accounts".to_string()),
        Network::Dev => Ok("https://faucet.devnet.rippletest.net/accounts".to_string()),
        Network::XahauTest => Ok("https://xahau-test.net/accounts".to_string()),
        Network::SidechainTest => Err(NetworkException::NoFaucetUrl(Network::SidechainTest)),
        Network::Custom(url) => Ok(url),
    }
}
