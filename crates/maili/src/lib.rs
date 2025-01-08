#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(any(feature = "full", feature = "std")), no_std)]

#[cfg(feature = "protocol")]
#[doc(inline)]
pub use maili_protocol as protocol;

#[cfg(feature = "registry")]
#[doc(inline)]
pub use maili_registry as registry;

#[cfg(feature = "provider")]
#[doc(inline)]
pub use maili_provider as provider;

#[cfg(feature = "rpc-types-engine")]
#[doc(inline)]
pub use maili_rpc_types_engine as rpc_types_engine;
