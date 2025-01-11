//! Extends l1 [`Transaction`] behavior.

use alloy_consensus::{Sealed, Transaction};

use crate::DepositTransaction;

/// Extends [`Transaction`] for Optimistic operations.
pub trait OpTransaction: Transaction {
    /// Deposit transaction.
    type DepositTx: DepositTransaction;

    /// Returns `true` if the transaction is a deposit transaction.
    fn is_deposit(&self) -> bool;

    /// Returns `true` if the transaction is a system transaction.
    fn is_system_transaction(&self) -> bool;

    /// Returns [`DepositTransaction`] if transaction is a deposit transaction.
    fn as_deposit(&self) -> Option<&Sealed<Self::DepositTx>>;
}
