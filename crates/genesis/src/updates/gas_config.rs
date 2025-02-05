//! The gas config update type.

use alloy_primitives::U256;
use alloy_sol_types::{sol, SolType};

use crate::{GasConfigUpdateError, RollupConfig, SystemConfig, SystemConfigLog};

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
}

impl TryFrom<&SystemConfigLog> for GasConfigUpdate {
    type Error = GasConfigUpdateError;

    fn try_from(sys_log: &SystemConfigLog) -> Result<Self, Self::Error> {
        let log = &sys_log.log;
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

        if sys_log.ecotone_active
            && RollupConfig::check_ecotone_l1_system_config_scalar(scalar.to_be_bytes()).is_err()
        {
            // ignore invalid scalars, retain the old system-config scalar
            return Ok(Self::default());
        }

        // If ecotone is active, set the overhead to zero, otherwise set to the decoded value.
        let overhead = if sys_log.ecotone_active { U256::ZERO } else { overhead };

        Ok(Self { scalar: Some(scalar), overhead: Some(overhead) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CONFIG_UPDATE_EVENT_VERSION_0, CONFIG_UPDATE_TOPIC};
    use alloy_primitives::{hex, uint, Address, Log, LogData, B256};

    #[test]
    fn test_gas_config_update_try_from() {
        let update_type = B256::ZERO;

        let log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    update_type,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000babe000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        let system_log = SystemConfigLog::new(log, false);
        let update = GasConfigUpdate::try_from(&system_log).unwrap();

        assert_eq!(update.overhead, Some(uint!(0xbabe_U256)));
        assert_eq!(update.scalar, Some(uint!(0xbeef_U256)));
    }
}
