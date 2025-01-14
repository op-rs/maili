//! Transaction envelope with support for OP [`TxDeposit`].

use alloy_consensus::Sealed;

use crate::TxDeposit;

/// Transaction envelope that encompasses a [`TxDeposit`].
pub trait DepositTxEnvelope {
    /// Returns `true` if the transaction is a deposit transaction.
    fn is_deposit(&self) -> bool;

    /// Returns [`TxDeposit`] if transaction is a deposit transaction.
    fn as_deposit(&self) -> Option<&Sealed<TxDeposit>>;
}
