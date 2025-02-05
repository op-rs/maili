//! The batcher update type.

use alloy_primitives::{Address, Log};
use alloy_sol_types::{sol, SolType};

use crate::{BatcherUpdateError, SystemConfig};

/// The batcher update type.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatcherUpdate {
    /// The batcher address.
    pub batcher_address: Address,
}

impl BatcherUpdate {
    /// Applies the update to the [`SystemConfig`].
    pub fn apply(&self, config: &mut SystemConfig) {
        config.batcher_address = self.batcher_address;
    }
}

impl TryFrom<Log> for BatcherUpdate {
    type Error = BatcherUpdateError;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        if log.data.data.len() != 96 {
            return Err(BatcherUpdateError::InvalidDataLen(log.data.data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log.data.data[0..32], true) else {
            return Err(BatcherUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(BatcherUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log.data.data[32..64], true) else {
            return Err(BatcherUpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(BatcherUpdateError::InvalidDataLength(length));
        }

        let Ok(batcher_address) = <sol!(address)>::abi_decode(&log.data.data[64..], true) else {
            return Err(BatcherUpdateError::BatcherAddressDecodingError);
        };

        Ok(Self { batcher_address })
    }
}
