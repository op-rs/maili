//! Transaction envelope with support for OP [`DepositTransaction`].

use alloy_consensus::Sealed;

use crate::DepositTransaction;

/// Transaction envelope that encompasses a [`DepositTransaction`].
pub trait DepositTxEnvelope {
    /// Deposit transaction.
    type DepositTx: DepositTransaction;

    /// Returns `true` if the transaction is a deposit transaction.
    fn is_deposit(&self) -> bool;

    /// Returns [`DepositTransaction`] if transaction is a deposit transaction.
    fn as_deposit(&self) -> Option<&Sealed<Self::DepositTx>>;
}
