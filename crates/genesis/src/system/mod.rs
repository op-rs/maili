//! Contains types related to the [`SystemConfig`].

use alloy_primitives::{b256, B256};

/// `keccak256("ConfigUpdate(uint256,uint8,bytes)")`
pub const CONFIG_UPDATE_TOPIC: B256 =
    b256!("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be");

/// The initial version of the system config event log.
pub const CONFIG_UPDATE_EVENT_VERSION_0: B256 = B256::ZERO;

mod batcher;
pub use batcher::BatcherUpdate;

mod gas_config;
pub use gas_config::GasConfigUpdate;

mod eip1559;
pub use eip1559::Eip1559Update;

mod operator_fee;
pub use operator_fee::OperatorFeeUpdate;

mod gas_limit;
pub use gas_limit::GasLimitUpdate;

mod config;
pub use config::SystemConfig;

mod log;
pub use log::SystemConfigLog;

mod update;
pub use update::SystemConfigUpdate;

mod kind;
pub use kind::SystemConfigUpdateKind;

mod errors;
pub use errors::{
    BatcherUpdateError, EIP1559UpdateError, GasConfigUpdateError, GasLimitUpdateError,
    LogProcessingError, OperatorFeeUpdateError, SystemConfigUpdateError,
};
