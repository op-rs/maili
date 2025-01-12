//! Extends l1 [`Transaction`] behavior.

use alloy_consensus::Sealed;

use crate::DepositTransaction;

/// Extends transaction envelope to encompass [`DepositTransaction`].
pub trait DepositTxEnvelope {
    /// Deposit transaction.
    type DepositTx: DepositTransaction;

    /// Returns envelope ID. Equivalent to [`Transaction::ty`](alloy_consensus::Transaction::ty).
    // todo: replace in favour of super trait <https://github.com/alloy-rs/alloy/pull/1910>
    fn id(&self) -> u8;

    /// Returns `true` if the transaction is a deposit transaction.
    fn is_deposit(&self) -> bool;

    /// Returns [`DepositTransaction`] if transaction is a deposit transaction.
    fn as_deposit(&self) -> Option<&Sealed<Self::DepositTx>>;
}
