//! The Operator Fee update type.

use alloy_sol_types::{sol, SolType};

use crate::{OperatorFeeUpdateError, SystemConfig, SystemConfigLog};

/// The Operator Fee update type.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OperatorFeeUpdate {
    /// The operator fee scalar.
    pub operator_fee_scalar: u32,
    /// The operator fee constant.
    pub operator_fee_constant: u64,
}

impl OperatorFeeUpdate {
    /// Applies the update to the [`SystemConfig`].
    pub fn apply(&self, config: &mut SystemConfig) {
        config.operator_fee_scalar = Some(self.operator_fee_scalar);
        config.operator_fee_constant = Some(self.operator_fee_constant);
    }
}

impl TryFrom<&SystemConfigLog> for OperatorFeeUpdate {
    type Error = OperatorFeeUpdateError;

    fn try_from(log: &SystemConfigLog) -> Result<Self, Self::Error> {
        let log = &log.log;
        if log.data.data.len() != 128 {
            return Err(OperatorFeeUpdateError::InvalidDataLen(log.data.data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log.data.data[0..32], true) else {
            return Err(OperatorFeeUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(OperatorFeeUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log.data.data[32..64], true) else {
            return Err(OperatorFeeUpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(OperatorFeeUpdateError::InvalidDataLength(length));
        }

        let Ok(operator_fee_scalar) = <sol!(uint32)>::abi_decode(&log.data.data[64..96], true)
        else {
            return Err(OperatorFeeUpdateError::ScalarDecodingError);
        };
        let Ok(operator_fee_constant) = <sol!(uint64)>::abi_decode(&log.data.data[96..], true)
        else {
            return Err(OperatorFeeUpdateError::ConstantDecodingError);
        };

        Ok(Self { operator_fee_scalar, operator_fee_constant })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CONFIG_UPDATE_EVENT_VERSION_0, CONFIG_UPDATE_TOPIC};
    use alloy_primitives::{hex, Address, Log, LogData, B256};

    #[test]
    fn test_operator_fee_update_try_from() {
        let update_type = B256::ZERO;

        let log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    update_type,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000babe000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        let system_log = SystemConfigLog::new(log, false);
        let update = OperatorFeeUpdate::try_from(&system_log).unwrap();

        assert_eq!(update.operator_fee_scalar, 0xbabe_u32);
        assert_eq!(update.operator_fee_constant, 0xbeef_u64);
    }
}
