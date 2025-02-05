//! The gas config update type.

use alloy_primitives::{Log, U256};
use alloy_sol_types::{sol, SolType};

use crate::{GasConfigUpdateError, RollupConfig, SystemConfig};

/// The gas config update type.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GasConfigUpdate {
    /// The scalar.
    pub scalar: Option<U256>,
    /// The overhead.
    pub overhead: Option<U256>,
}

impl GasConfigUpdate {
    /// Applies the update to the [`SystemConfig`].
    pub fn apply(&self, config: &mut SystemConfig) {
        if let Some(scalar) = self.scalar {
            config.scalar = scalar;
        }
        if let Some(overhead) = self.overhead {
            config.overhead = overhead;
        }
    }

    /// Attempts to construct a new gas config update from the log.
    pub fn try_new(log: Log, ecotone_active: bool) -> Result<Self, GasConfigUpdateError> {
        if log.data.data.len() != 128 {
            return Err(GasConfigUpdateError::InvalidDataLen(log.data.data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log.data.data[0..32], true) else {
            return Err(GasConfigUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(GasConfigUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log.data.data[32..64], true) else {
            return Err(GasConfigUpdateError::LengthDecodingError);
        };
        if length != 64 {
            return Err(GasConfigUpdateError::InvalidDataLength(length));
        }

        let Ok(overhead) = <sol!(uint256)>::abi_decode(&log.data.data[64..96], true) else {
            return Err(GasConfigUpdateError::OverheadDecodingError);
        };
        let Ok(scalar) = <sol!(uint256)>::abi_decode(&log.data.data[96..], true) else {
            return Err(GasConfigUpdateError::ScalarDecodingError);
        };

        if ecotone_active
            && RollupConfig::check_ecotone_l1_system_config_scalar(scalar.to_be_bytes()).is_err()
        {
            // ignore invalid scalars, retain the old system-config scalar
            return Ok(Self::default());
        }

        // If ecotone is active, set the overhead to zero, otherwise set to the decoded value.
        let overhead = if ecotone_active { U256::ZERO } else { overhead };

        Ok(Self { scalar: Some(scalar), overhead: Some(overhead) })
    }
}
