#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use alloy_primitives::Address;
use maili_genesis::ChainConfig;

/// A superchain configuration.
#[derive(Debug, Clone, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Superchain {
    /// Superchain identifier, without capitalization or display changes.
    pub name: String,
    /// Superchain configuration file contents.
    pub config: SuperchainConfig,
    /// Chain IDs of chains that are part of this superchain.
    pub chains: Vec<ChainConfig>,
}

/// A superchain configuration file format
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct SuperchainConfig {
    /// Superchain name (e.g. "Mainnet")
    pub name: String,
    /// Superchain L1 anchor information
    pub l1: SuperchainL1Info,
    /// Optional addresses for the superchain-wide default protocol versions contract.
    pub protocol_versions_addr: Option<Address>,
    /// Optional address for the superchain-wide default superchain config contract.
    pub superchain_config_addr: Option<Address>,
    /// The op contracts manager proxy address.
    #[serde(rename = "OPContractsManagerProxyAddr")]
    pub op_contracts_manager_proxy_addr: Option<Address>,
}

/// Superchain L1 anchor information
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct SuperchainL1Info {
    /// L1 chain ID
    // #[serde(rename = "ChainID")]
    pub chain_id: u64,
    /// L1 chain public RPC endpoint
    // #[serde(rename = "PublicRPC")]
    pub public_rpc: String,
    /// L1 chain explorer RPC endpoint
    pub explorer: String,
}

/// A list of Hydrated Superchain Configs.
#[derive(Debug, Clone, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Superchains {
    /// A list of superchain configs.
    pub superchains: Vec<Superchain>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{string::ToString, vec};

    #[test]
    fn test_superchain_serde() {
        let superchain = Superchain {
            name: "Mainnet".to_string(),
            config: SuperchainConfig {
                name: "Mainnet".to_string(),
                l1: SuperchainL1Info {
                    chain_id: 10,
                    public_rpc: "https://mainnet.rpc".to_string(),
                    explorer: "https://mainnet.explorer".to_string(),
                },
                protocol_versions_addr: None,
                superchain_config_addr: None,
                op_contracts_manager_proxy_addr: None,
            },
            chains: vec![ChainConfig::default()],
        };

        let serialized = serde_json::to_string(&superchain).unwrap();
        let deserialized: Superchain = serde_json::from_str(&serialized).unwrap();

        assert_eq!(superchain, deserialized);
    }
}
