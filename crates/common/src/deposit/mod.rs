//! Tramsaction types for Optimism.

mod source;
pub use source::{
    DepositSourceDomain, DepositSourceDomainIdentifier, L1InfoDepositSource, UpgradeDepositSource,
    UserDepositSource,
};

use alloy_primitives::B256;

/// A trait representing a deposit transaction with specific attributes.
pub trait DepositTransaction {
    /// Returns the hash that uniquely identifies the source of the deposit.
    ///
    /// # Returns
    /// An `Option<B256>` containing the source hash if available.
    fn source_hash(&self) -> Option<B256>;

    /// Returns the optional mint value of the deposit transaction.
    ///
    /// # Returns
    /// An `Option<u128>` representing the ETH value to mint on L2, if any.
    fn mint(&self) -> Option<u128>;

    /// Indicates whether the transaction is exempt from the L2 gas limit.
    ///
    /// # Returns
    /// A `bool` indicating if the transaction is a system transaction.
    fn is_system_transaction(&self) -> bool;
}
