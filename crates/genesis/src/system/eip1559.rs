//! The EIP-1559 update type.

use alloy_primitives::Log;
use alloy_sol_types::{sol, SolType};

use crate::{EIP1559UpdateError, SystemConfig};

/// The EIP-1559 update type.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eip1559Update {
    /// The EIP-1559 denominator.
    pub eip1559_denominator: u32,
    /// The EIP-1559 elasticity multiplier.
    pub eip1559_elasticity: u32,
}

impl Eip1559Update {
    /// Applies the update to the [`SystemConfig`].
    pub fn apply(&self, config: &mut SystemConfig) {
        config.eip1559_denominator = Some(self.eip1559_denominator);
        config.eip1559_elasticity = Some(self.eip1559_elasticity);
    }
}

impl TryFrom<Log> for Eip1559Update {
    type Error = EIP1559UpdateError;

    fn try_from(log: Log) -> Result<Self, Self::Error> {
        if log.data.data.len() != 96 {
            return Err(EIP1559UpdateError::InvalidDataLen(log.data.data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log.data.data[0..32], true) else {
            return Err(EIP1559UpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(EIP1559UpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log.data.data[32..64], true) else {
            return Err(EIP1559UpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(EIP1559UpdateError::InvalidDataLength(length));
        }

        let Ok(eip1559_params) = <sol!(uint64)>::abi_decode(&log.data.data[64..], true) else {
            return Err(EIP1559UpdateError::EIP1559DecodingError);
        };

        Ok(Self {
            eip1559_denominator: (eip1559_params >> 32) as u32,
            eip1559_elasticity: eip1559_params as u32,
        })
    }
}
