//! Contains the [`SystemConfig`] type and it's associated types.

use alloy_primitives::{b256, B256};

mod config;
pub use config::SystemConfig;

mod updates;
pub use updates::SystemConfigUpdateType;

mod errors;
pub use errors::SystemConfigUpdateError;

/// `keccak256("ConfigUpdate(uint256,uint8,bytes)")`
pub const CONFIG_UPDATE_TOPIC: B256 =
    b256!("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be");

/// The initial version of the system config event log.
pub const CONFIG_UPDATE_EVENT_VERSION_0: B256 = B256::ZERO;
