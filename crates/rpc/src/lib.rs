#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod api;

mod js_types;
pub use js_types::{
    Connectedness, Direction, Genesis, GossipScores, L1BlockRef, L2BlockRef, OutputResponse,
    PeerDump, PeerInfo, PeerScores, PeerStats, ProtocolVersion, ProtocolVersionError,
    ProtocolVersionFormatV0, ReqRespScores, RollupConfig, SafeHeadResponse, SuperchainSignal,
    SyncStatus, SystemConfig, TopicScores,
};
