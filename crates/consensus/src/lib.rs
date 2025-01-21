#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod hardforks;
pub use hardforks::{Ecotone, Fjord, Hardfork, Hardforks};

mod eip1559;
pub use eip1559::{
    decode_eip_1559_params, decode_holocene_extra_data, encode_holocene_extra_data,
    EIP1559ParamError,
};

mod deposit;
pub use deposit::{
    DepositContextDepositSource, DepositSourceDomain, DepositSourceDomainIdentifier,
    DepositTransaction, DepositTxEnvelope, L1InfoDepositSource, TxDeposit, UpgradeDepositSource,
    UserDepositSource, DEPOSIT_TX_TYPE_ID,
};

#[cfg(feature = "serde-bincode-compat")]
pub use deposit::serde_bincode_compat;

#[cfg(feature = "serde")]
pub use deposit::serde_deposit_tx_rpc;
