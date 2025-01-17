//! Contains the logic for creating a deposit context closing transaction for the interop hardfork.

use super::L1BlockInfoTx;
use crate::{
    info::variant::{L1_BLOCK_ADDRESS, L1_INFO_DEPOSITOR_ADDRESS},
    DepositContextDepositSource, DepositSourceDomain,
};
use alloy_consensus::Sealed;
use alloy_primitives::{Sealable, TxKind, U256};
use maili_consensus::TxDeposit;

/// `keccak256("depositsComplete()")[4:]`
const DEPOSIT_CONTEXT_SELECTOR: [u8; 4] = [0xe3, 0x2d, 0x20, 0xbb];

/// Create a [TxDeposit] for closing the deposit context. This deposit transaction, after
/// interop activation, always is placed last in the deposit section of the block.
///
/// <https://specs.optimism.io/interop/derivation.html#closing-the-deposit-context>
pub fn closing_deposit_context_tx(
    l1_info: &L1BlockInfoTx,
    sequence_number: u64,
) -> Sealed<TxDeposit> {
    let source = DepositSourceDomain::DepositContext(DepositContextDepositSource {
        l1_block_hash: l1_info.block_hash(),
        seq_number: sequence_number,
    });

    let deposit_tx = TxDeposit {
        source_hash: source.source_hash(),
        from: L1_INFO_DEPOSITOR_ADDRESS,
        to: TxKind::Call(L1_BLOCK_ADDRESS),
        mint: None,
        value: U256::ZERO,
        gas_limit: 36_000,
        is_system_transaction: false,
        input: DEPOSIT_CONTEXT_SELECTOR.into(),
    };

    deposit_tx.seal_slow()
}
