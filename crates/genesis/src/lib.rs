#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

mod accounts;
pub use accounts::SystemAccounts;

mod addresses;
pub use addresses::AddressList;

mod hardforks;
pub use hardforks::HardForkConfiguration;

mod altda;
pub use altda::AltDAConfig;

mod superchain;
pub use superchain::SuperchainLevel;

mod chain;
pub use chain::ChainConfig;

mod chain_ids;
pub use chain_ids::{
    BASE_MAINNET_CHAIN_ID, BASE_SEPOLIA_CHAIN_ID, OP_MAINNET_CHAIN_ID, OP_SEPOLIA_CHAIN_ID,
};

mod genesis;
pub use genesis::ChainGenesis;

mod params;
pub use params::{
    base_fee_params, base_fee_params_canyon, BaseFeeConfig, BASE_SEPOLIA_BASE_FEE_CONFIG,
    BASE_SEPOLIA_BASE_FEE_PARAMS, BASE_SEPOLIA_BASE_FEE_PARAMS_CANYON,
    BASE_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER, OP_MAINNET_BASE_FEE_CONFIG,
    OP_MAINNET_BASE_FEE_PARAMS, OP_MAINNET_BASE_FEE_PARAMS_CANYON,
    OP_MAINNET_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
    OP_MAINNET_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    OP_MAINNET_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER, OP_SEPOLIA_BASE_FEE_CONFIG,
    OP_SEPOLIA_BASE_FEE_PARAMS, OP_SEPOLIA_BASE_FEE_PARAMS_CANYON,
    OP_SEPOLIA_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
    OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    OP_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
};

mod roles;
pub use roles::Roles;

mod system;
pub use system::{
    BatcherUpdateError, EIP1559UpdateError, GasConfigUpdateError, GasLimitUpdateError,
    LogProcessingError, OperatorFeeUpdateError, SystemConfig, SystemConfigUpdateError,
    SystemConfigUpdateType, CONFIG_UPDATE_EVENT_VERSION_0, CONFIG_UPDATE_TOPIC,
};

mod rollup;
pub use rollup::{
    RollupConfig, FJORD_MAX_SEQUENCER_DRIFT, GRANITE_CHANNEL_TIMEOUT,
    MAX_RLP_BYTES_PER_CHANNEL_BEDROCK, MAX_RLP_BYTES_PER_CHANNEL_FJORD,
};
