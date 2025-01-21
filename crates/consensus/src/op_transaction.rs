//! Contains traits useful for abstracting over optimism transactions.

use crate::TxDeposit;
use alloy_consensus::Sealed;

/// This abstracts over a transaction that may be a deposit transaction type.
pub trait OpTransaction {
    /// Returns `true` if the transaction is a deposit transaction.
    fn is_deposit(&self) -> bool;

    /// Returns [`TxDeposit`] if transaction is a deposit transaction.
    fn as_deposit(&self) -> Option<&Sealed<TxDeposit>>;
}
