//! The gas limit update type.

use alloy_primitives::U64;
use alloy_sol_types::{sol, SolType};

use crate::{GasLimitUpdateError, SystemConfig, SystemConfigLog};

/// The gas limit update type.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GasLimitUpdate {
    /// The gas limit.
    pub gas_limit: u64,
}

impl GasLimitUpdate {
    /// Applies the update to the [`SystemConfig`].
    pub fn apply(&self, config: &mut SystemConfig) {
        config.gas_limit = self.gas_limit;
    }
}

impl TryFrom<&SystemConfigLog> for GasLimitUpdate {
    type Error = GasLimitUpdateError;

    fn try_from(log: &SystemConfigLog) -> Result<Self, Self::Error> {
        let log = &log.log;
        if log.data.data.len() != 96 {
            return Err(GasLimitUpdateError::InvalidDataLen(log.data.data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log.data.data[0..32], true) else {
            return Err(GasLimitUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(GasLimitUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log.data.data[32..64], true) else {
            return Err(GasLimitUpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(GasLimitUpdateError::InvalidDataLength(length));
        }

        let Ok(gas_limit) = <sol!(uint256)>::abi_decode(&log.data.data[64..], true) else {
            return Err(GasLimitUpdateError::GasLimitDecodingError);
        };

        Ok(Self { gas_limit: U64::from(gas_limit).saturating_to::<u64>() })
    }
}
