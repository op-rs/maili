//! Utility methods used by protocol types.

use alloc::vec::Vec;
use alloy_consensus::{Block, Transaction, TxType, Typed2718};
use alloy_primitives::B256;
use alloy_rlp::{Buf, Header};
use maili_consensus::DepositTxEnvelope;
use maili_genesis::{RollupConfig, SystemConfig};

use crate::{
    info::L1BlockInfoInterop, L1BlockInfoBedrock, L1BlockInfoEcotone, L1BlockInfoTx,
    OpBlockConversionError, SpanBatchError, SpanDecodingError,
};

/// Returns if the given `value` is a deposit transaction.
pub fn starts_with_2718_deposit<B>(value: &B) -> bool
where
    B: AsRef<[u8]>,
{
    value.as_ref().first() == Some(&0x7E)
}

/// Converts the OP [Block] to a partial [SystemConfig].
pub fn to_system_config<T>(
    block: &Block<T>,
    rollup_config: &RollupConfig,
) -> Result<SystemConfig, OpBlockConversionError>
where
    T: DepositTxEnvelope + Typed2718,
{
    if block.header.number == rollup_config.genesis.l2.number {
        if block.header.hash_slow() != rollup_config.genesis.l2.hash {
            return Err(OpBlockConversionError::InvalidGenesisHash(
                rollup_config.genesis.l2.hash,
                block.header.hash_slow(),
            ));
        }
        return rollup_config
            .genesis
            .system_config
            .ok_or(OpBlockConversionError::MissingSystemConfigGenesis);
    }

    if block.body.transactions.is_empty() {
        return Err(OpBlockConversionError::EmptyTransactions(block.header.hash_slow()));
    }
    let Some(tx) = block.body.transactions[0].as_deposit() else {
        return Err(OpBlockConversionError::InvalidTxType(block.body.transactions[0].ty()));
    };

    let l1_info = L1BlockInfoTx::decode_calldata(tx.input().as_ref())?;
    let l1_fee_scalar = match l1_info {
        L1BlockInfoTx::Bedrock(L1BlockInfoBedrock { l1_fee_scalar, .. }) => l1_fee_scalar,
        L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            base_fee_scalar,
            blob_base_fee_scalar,
            ..
        })
        | L1BlockInfoTx::Interop(L1BlockInfoInterop {
            base_fee_scalar,
            blob_base_fee_scalar,
            ..
        }) => {
            // Translate Ecotone values back into encoded scalar if needed.
            // We do not know if it was derived from a v0 or v1 scalar,
            // but v1 is fine, a 0 blob base fee has the same effect.
            let mut buf = B256::ZERO;
            buf[0] = 0x01;
            buf[24..28].copy_from_slice(blob_base_fee_scalar.to_be_bytes().as_ref());
            buf[28..32].copy_from_slice(base_fee_scalar.to_be_bytes().as_ref());
            buf.into()
        }
    };

    let mut cfg = SystemConfig {
        batcher_address: l1_info.batcher_address(),
        overhead: l1_info.l1_fee_overhead(),
        scalar: l1_fee_scalar,
        gas_limit: block.header.gas_limit,
        ..Default::default()
    };

    // After holocene's activation, the EIP-1559 parameters are stored in the block header's nonce.
    if rollup_config.is_holocene_active(block.header.timestamp) {
        let eip1559_params = block.header.nonce;
        cfg.eip1559_denominator = Some(u32::from_be_bytes(
            eip1559_params[0..4]
                .try_into()
                .map_err(|_| OpBlockConversionError::Eip1559DecodeError)?,
        ));
        cfg.eip1559_elasticity = Some(u32::from_be_bytes(
            eip1559_params[4..8]
                .try_into()
                .map_err(|_| OpBlockConversionError::Eip1559DecodeError)?,
        ));
    }

    Ok(cfg)
}

/// Reads transaction data from a reader.
pub fn read_tx_data(r: &mut &[u8]) -> Result<(Vec<u8>, TxType), SpanBatchError> {
    let mut tx_data = Vec::new();
    let first_byte =
        *r.first().ok_or(SpanBatchError::Decoding(SpanDecodingError::InvalidTransactionData))?;
    let mut tx_type = 0;
    if first_byte <= 0x7F {
        // EIP-2718: Non-legacy tx, so write tx type
        tx_type = first_byte;
        tx_data.push(tx_type);
        r.advance(1);
    }

    // Read the RLP header with a different reader pointer. This prevents the initial pointer from
    // being advanced in the case that what we read is invalid.
    let rlp_header = Header::decode(&mut (**r).as_ref())
        .map_err(|_| SpanBatchError::Decoding(SpanDecodingError::InvalidTransactionData))?;

    let tx_payload = if rlp_header.list {
        // Grab the raw RLP for the transaction data from `r`. It was unaffected since we copied it.
        let payload_length_with_header = rlp_header.payload_length + rlp_header.length();
        let payload = r[0..payload_length_with_header].to_vec();
        r.advance(payload_length_with_header);
        Ok(payload)
    } else {
        Err(SpanBatchError::Decoding(SpanDecodingError::InvalidTransactionData))
    }?;
    tx_data.extend_from_slice(&tx_payload);

    Ok((
        tx_data,
        tx_type
            .try_into()
            .map_err(|_| SpanBatchError::Decoding(SpanDecodingError::InvalidTransactionType))?,
    ))
}
