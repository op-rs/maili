//! System Accounts Module.

use alloy_primitives::{address, Address};

/// System accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SystemAccounts {
    /// The address that can deposit attributes
    pub attributes_depositor: Address,
    /// The address of the attributes predeploy
    pub attributes_predeploy: Address,
    /// The address of the fee vault
    pub fee_vault: Address,
}

impl Default for SystemAccounts {
    fn default() -> Self {
        Self {
            attributes_depositor: address!("deaddeaddeaddeaddeaddeaddeaddeaddead0001"),
            attributes_predeploy: address!("4200000000000000000000000000000000000015"),
            fee_vault: address!("4200000000000000000000000000000000000011"),
        }
    }
}
