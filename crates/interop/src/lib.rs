#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(any(feature = "std", feature = "interop")), no_std)]

extern crate alloc;

#[cfg(feature = "interop")]
mod supervisor;
#[cfg(feature = "interop")]
pub use supervisor::{Supervisor, SupervisorClient, SupervisorError};

mod root;
pub use root::{ChainRootInfo, OutputRootWithChain, SuperRoot, SuperRootResponse};

mod errors;
pub use errors::{SuperRootError, SuperRootResult};

mod safety;
pub use safety::SafetyLevel;

mod message;
pub use message::{
    extract_executing_messages, EnrichedExecutingMessage, ExecutingMessage, MessageIdentifier,
    MessagePayload,
};

mod derived;
pub use derived::DerivedIdPair;

mod constants;
pub use constants::{CROSS_L2_INBOX_ADDRESS, MESSAGE_EXPIRY_WINDOW, SUPER_ROOT_VERSION};
