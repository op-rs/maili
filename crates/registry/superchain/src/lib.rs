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
use maili_genesis::{ChainConfig, HardForkConfiguration};

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
pub struct SuperchainConfig {
    /// Superchain name (e.g. "Mainnet")
    pub name: String,
    /// Superchain L1 anchor information
    pub l1: SuperchainL1Info,
    /// Default hardforks timestamps.
    pub hardforks: HardForkConfiguration,
    /// Optional addresses for the superchain-wide default protocol versions contract.
    #[serde(alias = "protocolVersionsAddr")]
    pub protocol_versions_addr: Option<Address>,
    /// Optional address for the superchain-wide default superchain config contract.
    #[serde(alias = "superchainConfigAddr")]
    pub superchain_config_addr: Option<Address>,
    /// The op contracts manager proxy address.
    #[serde(alias = "OPContractsManagerProxyAddr")]
    pub op_contracts_manager_proxy_addr: Option<Address>,
}

/// Superchain L1 anchor information
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SuperchainL1Info {
    /// L1 chain ID
    #[serde(alias = "chainId")]
    pub chain_id: u64,
    /// L1 chain public RPC endpoint
    #[serde(alias = "publicRPC")]
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
        let raw: &str = r#"
        {
            "name": "Mainnet",
            "config": {
                "name": "Mainnet",
                "l1": {
                    "chainId": 10,
                    "publicRPC": "https://mainnet.rpc",
                    "explorer": "https://mainnet.explorer"
                },
                "hardforks": {
                    "canyon_time": 1699981200,
                    "delta_time": 1703203200,
                    "ecotone_time": 1708534800,
                    "fjord_time": 1716998400,
                    "granite_time": 1723478400,
                    "holocene_time": 1732633200
                }
            },
            "chains": []
        }
        "#;

        let superchain = Superchain {
            name: "Mainnet".to_string(),
            config: SuperchainConfig {
                name: "Mainnet".to_string(),
                l1: SuperchainL1Info {
                    chain_id: 10,
                    public_rpc: "https://mainnet.rpc".to_string(),
                    explorer: "https://mainnet.explorer".to_string(),
                },
                hardforks: HardForkConfiguration {
                    canyon_time: Some(1699981200),
                    delta_time: Some(1703203200),
                    ecotone_time: Some(1708534800),
                    fjord_time: Some(1716998400),
                    granite_time: Some(1723478400),
                    holocene_time: Some(1732633200),
                    isthmus_time: None,
                    interop_time: None,
                },
                protocol_versions_addr: None,
                superchain_config_addr: None,
                op_contracts_manager_proxy_addr: None,
            },
            chains: vec![],
        };

        let deserialized: Superchain = serde_json::from_str(raw).unwrap();
        assert_eq!(superchain, deserialized);
    }
}
