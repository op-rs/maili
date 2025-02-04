//! HardFork configuration module.

/// Hardfork Configuration.
///
/// Contains the activation times for all hardforks.
#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HardForkConfiguration {
    /// Canyon hardfork activation time
    pub canyon_time: Option<u64>,
    /// Delta hardfork activation time
    pub delta_time: Option<u64>,
    /// Ecotone hardfork activation time
    pub ecotone_time: Option<u64>,
    /// Fjord hardfork activation time
    pub fjord_time: Option<u64>,
    /// Granite hardfork activation time
    pub granite_time: Option<u64>,
    /// Holocene hardfork activation time
    pub holocene_time: Option<u64>,
    /// Isthmus hardfork activation time
    pub isthmus_time: Option<u64>,
    /// Interop hardfork activation time
    pub interop_time: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardfork_config_serde_full() {
        let serialized = r#"{
            "canyon_time":1,
            "delta_time":2,
            "ecotone_time":3,
            "fjord_time":4,
            "granite_time":5,
            "holocene_time":6,
            "isthmus_time":7,
            "interop_time":8
        }"#;

        let deserialized: HardForkConfiguration = serde_json::from_str(serialized).unwrap();
        assert_eq!(
            deserialized,
            HardForkConfiguration {
                canyon_time: Some(1),
                delta_time: Some(2),
                ecotone_time: Some(3),
                fjord_time: Some(4),
                granite_time: Some(5),
                holocene_time: Some(6),
                isthmus_time: Some(7),
                interop_time: Some(8)
            }
        );
    }
}
