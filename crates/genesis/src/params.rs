//! Module containing fee parameters.

use alloy_eips::eip1559::BaseFeeParams;

use crate::{
    BASE_MAINNET_CHAIN_ID, BASE_SEPOLIA_CHAIN_ID, OP_MAINNET_CHAIN_ID, OP_SEPOLIA_CHAIN_ID,
};

/// Base fee max change denominator for Optimism Mainnet as defined in the Optimism
/// [transaction costs](https://community.optimism.io/docs/developers/build/differences/#transaction-costs) doc.
pub const OP_MAINNET_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR: u128 = 50;

/// Base fee max change denominator for Optimism Mainnet as defined in the Optimism Canyon
/// hardfork.
pub const OP_MAINNET_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON: u128 = 250;

/// Base fee max change denominator for Optimism Mainnet as defined in the Optimism
/// [transaction costs](https://community.optimism.io/docs/developers/build/differences/#transaction-costs) doc.
pub const OP_MAINNET_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER: u128 = 6;

/// Base fee max change denominator for Optimism Sepolia as defined in the Optimism
/// [transaction costs](https://community.optimism.io/docs/developers/build/differences/#transaction-costs) doc.
pub const OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR: u128 = 50;

/// Base fee max change denominator for Optimism Sepolia as defined in the Optimism Canyon
/// hardfork.
pub const OP_SEPOLIA_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON: u128 = 250;

/// Base fee max change denominator for Optimism Sepolia as defined in the Optimism
/// [transaction costs](https://community.optimism.io/docs/developers/build/differences/#transaction-costs) doc.
pub const OP_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER: u128 = 6;

/// Base fee max change denominator for Base Sepolia as defined in the Optimism
/// [transaction costs](https://community.optimism.io/docs/developers/build/differences/#transaction-costs) doc.
pub const BASE_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER: u128 = 10;

/// Get the base fee parameters for Optimism Sepolia.
pub const OP_SEPOLIA_BASE_FEE_PARAMS: BaseFeeParams = BaseFeeParams {
    max_change_denominator: OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    elasticity_multiplier: OP_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
};

/// Get the base fee parameters for Base Sepolia.
pub const BASE_SEPOLIA_BASE_FEE_PARAMS: BaseFeeParams = BaseFeeParams {
    max_change_denominator: OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    elasticity_multiplier: BASE_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
};

/// Get the base fee parameters for Optimism Mainnet.
pub const OP_MAINNET_BASE_FEE_PARAMS: BaseFeeParams = BaseFeeParams {
    max_change_denominator: OP_MAINNET_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    elasticity_multiplier: OP_MAINNET_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
};

/// Get the base fee parameters for Optimism Sepolia.
pub const OP_SEPOLIA_BASE_FEE_PARAMS_CANYON: BaseFeeParams = BaseFeeParams {
    max_change_denominator: OP_SEPOLIA_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
    elasticity_multiplier: OP_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
};

/// Get the base fee parameters for Base Sepolia.
pub const BASE_SEPOLIA_BASE_FEE_PARAMS_CANYON: BaseFeeParams = BaseFeeParams {
    max_change_denominator: OP_SEPOLIA_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
    elasticity_multiplier: BASE_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
};

/// Get the base fee parameters for Optimism Mainnet.
pub const OP_MAINNET_BASE_FEE_PARAMS_CANYON: BaseFeeParams = BaseFeeParams {
    max_change_denominator: OP_MAINNET_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
    elasticity_multiplier: OP_MAINNET_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
};

/// Returns the [`BaseFeeParams`] for the given chain id.
pub const fn base_fee_params(chain_id: u64) -> BaseFeeParams {
    match chain_id {
        OP_MAINNET_CHAIN_ID => OP_MAINNET_BASE_FEE_PARAMS,
        OP_SEPOLIA_CHAIN_ID => OP_SEPOLIA_BASE_FEE_PARAMS,
        BASE_MAINNET_CHAIN_ID => OP_MAINNET_BASE_FEE_PARAMS,
        BASE_SEPOLIA_CHAIN_ID => BASE_SEPOLIA_BASE_FEE_PARAMS,
        _ => OP_MAINNET_BASE_FEE_PARAMS,
    }
}

/// Returns the [`BaseFeeParams`] for the given chain id, for canyon hardfork.
pub const fn base_fee_params_canyon(chain_id: u64) -> BaseFeeParams {
    match chain_id {
        OP_MAINNET_CHAIN_ID => OP_MAINNET_BASE_FEE_PARAMS_CANYON,
        OP_SEPOLIA_CHAIN_ID => OP_SEPOLIA_BASE_FEE_PARAMS_CANYON,
        BASE_MAINNET_CHAIN_ID => OP_MAINNET_BASE_FEE_PARAMS_CANYON,
        BASE_SEPOLIA_CHAIN_ID => BASE_SEPOLIA_BASE_FEE_PARAMS_CANYON,
        _ => OP_MAINNET_BASE_FEE_PARAMS_CANYON,
    }
}

/// Get the base fee parameters for Optimism Sepolia.
pub const OP_SEPOLIA_BASE_FEE_CONFIG: BaseFeeConfig = BaseFeeConfig {
    eip1559_elasticity: OP_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
    eip1559_denominator: OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    eip1559_denominator_canyon: OP_SEPOLIA_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
};

/// Get the base fee parameters for Base Sepolia.
pub const BASE_SEPOLIA_BASE_FEE_CONFIG: BaseFeeConfig = BaseFeeConfig {
    eip1559_elasticity: BASE_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
    eip1559_denominator: OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    eip1559_denominator_canyon: OP_SEPOLIA_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
};

/// Get the base fee parameters for Optimism Mainnet.
pub const OP_MAINNET_BASE_FEE_CONFIG: BaseFeeConfig = BaseFeeConfig {
    eip1559_elasticity: OP_MAINNET_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
    eip1559_denominator: OP_MAINNET_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
    eip1559_denominator_canyon: OP_MAINNET_EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR_CANYON,
};

/// Optimism Base Fee Configuration
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseFeeConfig {
    /// EIP 1559 Elasticity Parameter
    #[cfg_attr(
        feature = "serde",
        serde(rename = "eip1559Elasticity", alias = "eip1559_elasticity")
    )]
    #[cfg_attr(feature = "serde", serde(with = "maili_serde::quantity"))]
    pub eip1559_elasticity: u128,
    /// EIP 1559 Denominator
    #[cfg_attr(
        feature = "serde",
        serde(rename = "eip1559Denominator", alias = "eip1559_denominator")
    )]
    #[cfg_attr(feature = "serde", serde(with = "maili_serde::quantity"))]
    pub eip1559_denominator: u128,
    /// EIP 1559 Denominator for the Canyon hardfork
    #[cfg_attr(
        feature = "serde",
        serde(rename = "eip1559DenominatorCanyon", alias = "eip1559_denominator_canyon")
    )]
    #[cfg_attr(feature = "serde", serde(with = "maili_serde::quantity"))]
    pub eip1559_denominator_canyon: u128,
}

impl BaseFeeConfig {
    /// Returns the inner [BaseFeeParams].
    pub const fn as_base_fee_params(&self) -> BaseFeeParams {
        BaseFeeParams {
            max_change_denominator: self.eip1559_denominator,
            elasticity_multiplier: self.eip1559_elasticity,
        }
    }

    /// Returns the [BaseFeeParams] for the canyon hardfork.
    pub const fn as_canyon_base_fee_params(&self) -> BaseFeeParams {
        BaseFeeParams {
            max_change_denominator: self.eip1559_denominator_canyon,
            elasticity_multiplier: self.eip1559_elasticity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_fee_params_from_chain_id() {
        assert_eq!(base_fee_params(OP_MAINNET_CHAIN_ID), OP_MAINNET_BASE_FEE_PARAMS);
        assert_eq!(base_fee_params(OP_SEPOLIA_CHAIN_ID), OP_SEPOLIA_BASE_FEE_PARAMS);
        assert_eq!(base_fee_params(BASE_MAINNET_CHAIN_ID), OP_MAINNET_BASE_FEE_PARAMS);
        assert_eq!(base_fee_params(BASE_SEPOLIA_CHAIN_ID), BASE_SEPOLIA_BASE_FEE_PARAMS);
        assert_eq!(base_fee_params(0), OP_MAINNET_BASE_FEE_PARAMS);
    }

    #[test]
    fn test_base_fee_params_canyon_from_chain_id() {
        assert_eq!(base_fee_params_canyon(OP_MAINNET_CHAIN_ID), OP_MAINNET_BASE_FEE_PARAMS_CANYON);
        assert_eq!(base_fee_params_canyon(OP_SEPOLIA_CHAIN_ID), OP_SEPOLIA_BASE_FEE_PARAMS_CANYON);
        assert_eq!(
            base_fee_params_canyon(BASE_MAINNET_CHAIN_ID),
            OP_MAINNET_BASE_FEE_PARAMS_CANYON
        );
        assert_eq!(
            base_fee_params_canyon(BASE_SEPOLIA_CHAIN_ID),
            BASE_SEPOLIA_BASE_FEE_PARAMS_CANYON
        );
        assert_eq!(base_fee_params_canyon(0), OP_MAINNET_BASE_FEE_PARAMS_CANYON);
    }

    #[test]
    fn test_base_fee_config_ser() {
        let config = OP_MAINNET_BASE_FEE_CONFIG;
        let raw_str = serde_json::to_string(&config).unwrap();
        assert_eq!(
            raw_str,
            r#"{"eip1559Elasticity":"0x6","eip1559Denominator":"0x32","eip1559DenominatorCanyon":"0xfa"}"#
        );
    }

    #[test]
    fn test_base_fee_config_serde_strs() {
        let raw_str: &'static str = r#"{"eip1559Elasticity":"6","eip1559Denominator":"50","eip1559DenominatorCanyon":"250"}"#;
        let config: BaseFeeConfig = serde_json::from_str(raw_str).unwrap();
        assert_eq!(config, OP_MAINNET_BASE_FEE_CONFIG);
    }

    #[test]
    fn test_base_fee_config_serde_raw_number() {
        let raw_str: &'static str =
            r#"{"eip1559Elasticity":6,"eip1559Denominator":50,"eip1559DenominatorCanyon":250}"#;
        let config: BaseFeeConfig = serde_json::from_str(raw_str).unwrap();
        assert_eq!(config, OP_MAINNET_BASE_FEE_CONFIG);
    }

    #[test]
    fn test_base_fee_config_serde_hex() {
        let raw_str: &'static str = r#"{"eip1559Elasticity":"0x6","eip1559Denominator":"0x32","eip1559DenominatorCanyon":"0xfa"}"#;
        let config: BaseFeeConfig = serde_json::from_str(raw_str).unwrap();
        assert_eq!(config, OP_MAINNET_BASE_FEE_CONFIG);
    }
}
