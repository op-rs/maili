//! Extends l1 [`Transaction`] behavior.

use alloy_consensus::{Sealed, Typed2718};

use crate::DepositTransaction;

/// Extends transaction envelope to encompass [`DepositTransaction`].
pub trait DepositTxEnvelope: Typed2718 {
    /// Deposit transaction.
    type DepositTx: DepositTransaction;

    /// Returns `true` if the transaction is a deposit transaction.
    fn is_deposit(&self) -> bool;

    /// Returns [`DepositTransaction`] if transaction is a deposit transaction.
    fn as_deposit(&self) -> Option<&Sealed<Self::DepositTx>>;
}
