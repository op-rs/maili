//! Contains the AltDA Configuration.

use alloy_primitives::Address;

/// AltDA configuration.
#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AltDAConfig {
    /// AltDA challenge address
    pub da_challenge_address: Option<Address>,
    /// AltDA challenge window time (in seconds)
    pub da_challenge_window: Option<u64>,
    /// AltDA resolution window time (in seconds)
    pub da_resolve_window: Option<u64>,
}
