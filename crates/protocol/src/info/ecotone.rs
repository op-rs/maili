//! Contains ecotone-specific L1 block info types.

use alloc::{format, string::ToString, vec::Vec};
use alloy_primitives::{Address, Bytes, B256, U256};

use crate::{estimate_fjord_tx_size, DecodeError};

const ZERO_BYTE_COST: u64 = 4;
const NON_ZERO_BYTE_COST: u64 = 16;

/// Represents the fields within an Ecotone L1 block info transaction.
///
/// Ecotone Binary Format
/// +---------+--------------------------+
/// | Bytes   | Field                    |
/// +---------+--------------------------+
/// | 4       | Function signature       |
/// | 4       | BaseFeeScalar            |
/// | 4       | BlobBaseFeeScalar        |
/// | 8       | SequenceNumber           |
/// | 8       | Timestamp                |
/// | 8       | L1BlockNumber            |
/// | 32      | BaseFee                  |
/// | 32      | BlobBaseFee              |
/// | 32      | BlockHash                |
/// | 32      | BatcherHash              |
/// +---------+--------------------------+
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct L1BlockInfoEcotone {
    /// The current L1 origin block number
    pub number: u64,
    /// The current L1 origin block's timestamp
    pub time: u64,
    /// The current L1 origin block's basefee
    pub base_fee: u64,
    /// The current L1 origin block's hash
    pub block_hash: B256,
    /// The current sequence number
    pub sequence_number: u64,
    /// The address of the batch submitter
    pub batcher_address: Address,
    /// The current blob base fee on L1
    pub blob_base_fee: u128,
    /// The fee scalar for L1 blobspace data
    pub blob_base_fee_scalar: u32,
    /// The fee scalar for L1 data
    pub base_fee_scalar: u32,
}

impl L1BlockInfoEcotone {
    /// The type byte identifier for the L1 scalar format in Ecotone.
    pub const L1_SCALAR: u8 = 1;

    /// The length of an L1 info transaction in Ecotone.
    pub const L1_INFO_TX_LEN: usize = 4 + 32 * 5;

    /// The 4 byte selector of "setL1BlockValuesEcotone()"
    pub const L1_INFO_TX_SELECTOR: [u8; 4] = [0x44, 0x0a, 0x5e, 0x20];

    /// Encodes the [L1BlockInfoEcotone] object into Ethereum transaction calldata.
    pub fn encode_calldata(&self) -> Bytes {
        let mut buf = Vec::with_capacity(Self::L1_INFO_TX_LEN);
        buf.extend_from_slice(Self::L1_INFO_TX_SELECTOR.as_ref());
        buf.extend_from_slice(self.base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.blob_base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.sequence_number.to_be_bytes().as_ref());
        buf.extend_from_slice(self.time.to_be_bytes().as_ref());
        buf.extend_from_slice(self.number.to_be_bytes().as_ref());
        buf.extend_from_slice(U256::from(self.base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(U256::from(self.blob_base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(self.block_hash.as_ref());
        buf.extend_from_slice(self.batcher_address.into_word().as_ref());
        buf.into()
    }

    /// Calculates the L1 fee for transaction data post-ecotone.
    ///
    /// l1BaseFee * 16 * l1BaseFeeScalar + l1BlobBaseFee * l1BlobBaseFeeScalar
    pub fn calculate_l1_fee_scaled(&self) -> U256 {
        let calldata_cost_per_byte = U256::from(self.base_fee)
            * U256::from(NON_ZERO_BYTE_COST)
            * U256::from(self.base_fee_scalar);
        let blob_cost_per_byte =
            U256::from(self.blob_base_fee) * U256::from(self.blob_base_fee_scalar);

        calldata_cost_per_byte + blob_cost_per_byte
    }

    /// Calculates the cost to post a transaction's to L1 after the ecotone hardfork.
    ///
    /// This method is for *before* the FJORD hardfork.
    /// After the FJORD hardfork, [`L1BlockInfoEcotone::calculate_tx_l1_cost_fjord`]
    /// must be used.
    pub fn calculate_tx_l1_cost(&self, input: &[u8]) -> U256 {
        let l1_fee_scaled = self.calculate_l1_fee_scaled();

        let rollup_data_gas_cost = U256::from(input.iter().fold(0, |acc, byte| {
            acc + if *byte == 0x00 { ZERO_BYTE_COST } else { NON_ZERO_BYTE_COST }
        }));

        l1_fee_scaled * rollup_data_gas_cost / U256::from(1_000_000 * NON_ZERO_BYTE_COST)
    }

    /// Calculates the cost to post a transaction's data to L1 after the FJORD hardfork.
    pub fn calculate_tx_l1_cost_fjord(&self, input: &[u8]) -> U256 {
        let l1_fee_scaled = self.calculate_l1_fee_scaled();

        let rollup_data_gas_cost =
            estimate_fjord_tx_size(input) * U256::from(NON_ZERO_BYTE_COST) / U256::from(1_000_000);

        l1_fee_scaled * rollup_data_gas_cost / U256::from(1_000_000 * NON_ZERO_BYTE_COST)
    }

    /// Decodes the [L1BlockInfoEcotone] object from ethereum transaction calldata.
    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> {
        if r.len() != Self::L1_INFO_TX_LEN {
            return Err(DecodeError::InvalidLength(format!(
                "Invalid calldata length for Ecotone L1 info transaction, expected {}, got {}",
                Self::L1_INFO_TX_LEN,
                r.len()
            )));
        }
        let base_fee_scalar = u32::from_be_bytes(r[4..8].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for base fee scalar".to_string())
        })?);
        let blob_base_fee_scalar = u32::from_be_bytes(r[8..12].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for blob base fee scalar".to_string())
        })?);
        let sequence_number = u64::from_be_bytes(r[12..20].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for sequence number".to_string())
        })?);
        let timestamp =
            u64::from_be_bytes(r[20..28].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for timestamp".to_string())
            })?);
        let l1_block_number = u64::from_be_bytes(r[28..36].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for L1 block number".to_string())
        })?);
        let base_fee =
            u64::from_be_bytes(r[60..68].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for base fee".to_string())
            })?);
        let blob_base_fee = u128::from_be_bytes(r[84..100].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for blob base fee".to_string())
        })?);
        let block_hash = B256::from_slice(r[100..132].as_ref());
        let batcher_address = Address::from_slice(r[144..164].as_ref());

        Ok(Self {
            number: l1_block_number,
            time: timestamp,
            base_fee,
            block_hash,
            sequence_number,
            batcher_address,
            blob_base_fee,
            blob_base_fee_scalar,
            base_fee_scalar,
        })
    }
}
