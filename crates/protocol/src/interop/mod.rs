//! Interop message primitives.
//!
//! <https://specs.optimism.io/interop>

mod graph;
pub use graph::MessageGraph;

mod message;
pub use message::{
    extract_executing_messages, EnrichedExecutingMessage, ExecutingMessage, MessageIdentifier,
    MessagePayload,
};

mod safety;
pub use safety::SafetyLevel;

mod constants;
pub use constants::{CROSS_L2_INBOX_ADDRESS, MESSAGE_EXPIRY_WINDOW, SUPER_ROOT_VERSION};

mod traits;
pub use traits::InteropProvider;

mod errors;
pub use errors::{
    InteropProviderError, InteropProviderResult, MessageGraphError, MessageGraphResult,
    SuperRootError, SuperRootResult,
};

mod super_root;
pub use super_root::{OutputRootWithChain, SuperRoot};

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
