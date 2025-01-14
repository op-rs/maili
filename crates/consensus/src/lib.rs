#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

mod deposit;
#[cfg(feature = "serde-bincode-compat")]
pub use deposit::serde_bincode_compat;
#[cfg(feature = "serde")]
pub use deposit::serde_deposit_tx_rpc;
pub use deposit::{
    DepositSourceDomain, DepositSourceDomainIdentifier, DepositTransaction, DepositTxEnvelope,
    L1InfoDepositSource, TxDeposit, UpgradeDepositSource, UserDepositSource, DEPOSIT_TX_TYPE_ID,
};
