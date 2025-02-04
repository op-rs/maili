//! System Config Type



use crate::RollupConfig;





/// An error occurred while processing the update log.
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LogProcessingError {
    /// Received an incorrect number of log topics.
    #[error("Invalid config update log: invalid topic length: {0}")]
    InvalidTopicLen(usize),
    /// The log topic is invalid.
    #[error("Invalid config update log: invalid topic")]
    InvalidTopic,
    /// The config update log version is unsupported.
    #[error("Invalid config update log: unsupported version: {0}")]
    UnsupportedVersion(B256),
    /// Failed to decode the update type from the config update log.
    #[error("Failed to decode config update log: update type")]
    UpdateTypeDecodingError,
    /// An invalid system config update type.
    #[error("Invalid system config update type: {0}")]
    InvalidSystemConfigUpdateType(u64),
}

/// An error for updating the batcher address on the [SystemConfig].
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BatcherUpdateError {
    /// Invalid data length.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLen(usize),
    /// Failed to decode the data pointer argument from the batcher update log.
    #[error("Failed to decode batcher update log: data pointer")]
    PointerDecodingError,
    /// The data pointer is invalid.
    #[error("Invalid config update log: invalid data pointer: {0}")]
    InvalidDataPointer(u64),
    /// Failed to decode the data length argument from the batcher update log.
    #[error("Failed to decode batcher update log: data length")]
    LengthDecodingError,
    /// The data length is invalid.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLength(u64),
    /// Failed to decode the batcher address argument from the batcher update log.
    #[error("Failed to decode batcher update log: batcher address")]
    BatcherAddressDecodingError,
}

/// An error for updating the gas config on the [SystemConfig].
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GasConfigUpdateError {
    /// Invalid data length.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLen(usize),
    /// Failed to decode the data pointer argument from the gas config update log.
    #[error("Failed to decode gas config update log: data pointer")]
    PointerDecodingError,
    /// The data pointer is invalid.
    #[error("Invalid config update log: invalid data pointer: {0}")]
    InvalidDataPointer(u64),
    /// Failed to decode the data length argument from the gas config update log.
    #[error("Failed to decode gas config update log: data length")]
    LengthDecodingError,
    /// The data length is invalid.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLength(u64),
    /// Failed to decode the overhead argument from the gas config update log.
    #[error("Failed to decode gas config update log: overhead")]
    OverheadDecodingError,
    /// Failed to decode the scalar argument from the gas config update log.
    #[error("Failed to decode gas config update log: scalar")]
    ScalarDecodingError,
}

/// An error for updating the gas limit on the [SystemConfig].
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GasLimitUpdateError {
    /// Invalid data length.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLen(usize),
    /// Failed to decode the data pointer argument from the gas limit update log.
    #[error("Failed to decode gas limit update log: data pointer")]
    PointerDecodingError,
    /// The data pointer is invalid.
    #[error("Invalid config update log: invalid data pointer: {0}")]
    InvalidDataPointer(u64),
    /// Failed to decode the data length argument from the gas limit update log.
    #[error("Failed to decode gas limit update log: data length")]
    LengthDecodingError,
    /// The data length is invalid.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLength(u64),
    /// Failed to decode the gas limit argument from the gas limit update log.
    #[error("Failed to decode gas limit update log: gas limit")]
    GasLimitDecodingError,
}

/// An error for updating the EIP-1559 parameters on the [SystemConfig].
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EIP1559UpdateError {
    /// Invalid data length.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLen(usize),
    /// Failed to decode the data pointer argument from the eip 1559 update log.
    #[error("Failed to decode eip1559 parameter update log: data pointer")]
    PointerDecodingError,
    /// The data pointer is invalid.
    #[error("Invalid config update log: invalid data pointer: {0}")]
    InvalidDataPointer(u64),
    /// Failed to decode the data length argument from the eip 1559 update log.
    #[error("Failed to decode eip1559 parameter update log: data length")]
    LengthDecodingError,
    /// The data length is invalid.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLength(u64),
    /// Failed to decode the eip1559 params argument from the eip 1559 update log.
    #[error("Failed to decode eip1559 parameter update log: eip1559 parameters")]
    EIP1559DecodingError,
}

/// An error for updating the operator fee parameters on the [SystemConfig].
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OperatorFeeUpdateError {
    /// Invalid data length.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLen(usize),
    /// Failed to decode the data pointer argument from the eip 1559 update log.
    #[error("Failed to decode eip1559 parameter update log: data pointer")]
    PointerDecodingError,
    /// The data pointer is invalid.
    #[error("Invalid config update log: invalid data pointer: {0}")]
    InvalidDataPointer(u64),
    /// Failed to decode the data length argument from the eip 1559 update log.
    #[error("Failed to decode eip1559 parameter update log: data length")]
    LengthDecodingError,
    /// The data length is invalid.
    #[error("Invalid config update log: invalid data length: {0}")]
    InvalidDataLength(u64),
    /// Failed to decode the scalar argument from the update log.
    #[error("Failed to decode operator fee parameter update log: scalar")]
    ScalarDecodingError,
    /// Failed to decode the constant argument from the update log.
    #[error("Failed to decode operator fee parameter update log: constant")]
    ConstantDecodingError,
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    use alloy_primitives::{address, b256, hex, LogData, B256};
    use arbitrary::Arbitrary;
    use rand::Rng;

    #[test]
    fn test_system_config_alias() {
        let sc_str: &'static str = r#"{
          "batcherAddress": "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985",
          "overhead": "0x00000000000000000000000000000000000000000000000000000000000000bc",
          "scalar": "0x00000000000000000000000000000000000000000000000000000000000a6fe0",
          "gasLimit": 30000000
        }"#;
        let system_config: SystemConfig = serde_json::from_str(sc_str).unwrap();
        assert_eq!(
            system_config,
            SystemConfig {
                batcher_address: address!("6887246668a3b87F54DeB3b94Ba47a6f63F32985"),
                overhead: U256::from(0xbc),
                scalar: U256::from(0xa6fe0),
                gas_limit: 30000000,
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_arbitrary_system_config() {
        let mut bytes = [0u8; 1024];
        rand::rng().fill(bytes.as_mut_slice());
        SystemConfig::arbitrary(&mut arbitrary::Unstructured::new(&bytes)).unwrap();
    }

    #[test]
    fn test_eip_1559_params_from_system_config_none() {
        let rollup_config = RollupConfig::default();
        let sys_config = SystemConfig::default();
        assert_eq!(sys_config.eip_1559_params(&rollup_config, 0, 0), None);
    }

    #[test]
    fn test_eip_1559_params_from_system_config_some() {
        let rollup_config = RollupConfig { holocene_time: Some(0), ..Default::default() };
        let sys_config = SystemConfig {
            eip1559_denominator: Some(1),
            eip1559_elasticity: None,
            ..Default::default()
        };
        let expected = Some(B64::from_slice(&[1u32.to_be_bytes(), 0u32.to_be_bytes()].concat()));
        assert_eq!(sys_config.eip_1559_params(&rollup_config, 0, 0), expected);
    }

    #[test]
    fn test_eip_1559_params_from_system_config() {
        let rollup_config = RollupConfig { holocene_time: Some(0), ..Default::default() };
        let sys_config = SystemConfig {
            eip1559_denominator: Some(1),
            eip1559_elasticity: Some(2),
            ..Default::default()
        };
        let expected = Some(B64::from_slice(&[1u32.to_be_bytes(), 2u32.to_be_bytes()].concat()));
        assert_eq!(sys_config.eip_1559_params(&rollup_config, 0, 0), expected);
    }

    #[test]
    fn test_default_eip_1559_params_from_system_config() {
        let rollup_config = RollupConfig { holocene_time: Some(0), ..Default::default() };
        let sys_config = SystemConfig {
            eip1559_denominator: None,
            eip1559_elasticity: None,
            ..Default::default()
        };
        let expected = Some(B64::ZERO);
        assert_eq!(sys_config.eip_1559_params(&rollup_config, 0, 0), expected);
    }

    #[test]
    fn test_default_eip_1559_params_from_system_config_pre_holocene() {
        let rollup_config = RollupConfig::default();
        let sys_config = SystemConfig {
            eip1559_denominator: Some(1),
            eip1559_elasticity: Some(2),
            ..Default::default()
        };
        assert_eq!(sys_config.eip_1559_params(&rollup_config, 0, 0), None);
    }

    #[test]
    fn test_default_eip_1559_params_first_block_holocene() {
        let rollup_config = RollupConfig { holocene_time: Some(2), ..Default::default() };
        let sys_config = SystemConfig {
            eip1559_denominator: Some(1),
            eip1559_elasticity: Some(2),
            ..Default::default()
        };
        assert_eq!(sys_config.eip_1559_params(&rollup_config, 0, 2), Some(B64::ZERO));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_system_config_serde() {
        let sc_str = r#"{
          "batcherAddr": "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985",
          "overhead": "0x00000000000000000000000000000000000000000000000000000000000000bc",
          "scalar": "0x00000000000000000000000000000000000000000000000000000000000a6fe0",
          "gasLimit": 30000000
        }"#;
        let system_config: SystemConfig = serde_json::from_str(sc_str).unwrap();
        assert_eq!(
            system_config.batcher_address,
            address!("6887246668a3b87F54DeB3b94Ba47a6f63F32985")
        );
        assert_eq!(system_config.overhead, U256::from(0xbc));
        assert_eq!(system_config.scalar, U256::from(0xa6fe0));
        assert_eq!(system_config.gas_limit, 30000000);
    }

    #[test]
    fn test_system_config_update_with_receipts_unchanged() {
        let mut system_config = SystemConfig::default();
        let receipts = vec![];
        let l1_system_config_address = Address::ZERO;
        let ecotone_active = false;

        system_config
            .update_with_receipts(&receipts, l1_system_config_address, ecotone_active)
            .unwrap();

        assert_eq!(system_config, SystemConfig::default());
    }

    #[test]
    fn test_system_config_update_with_receipts_batcher_address() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000000");
        let mut system_config = SystemConfig::default();
        let l1_system_config_address = Address::ZERO;
        let ecotone_active = false;

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        let receipt = Receipt {
            logs: vec![update_log],
            status: Eip658Value::Eip658(true),
            cumulative_gas_used: 0,
        };

        system_config
            .update_with_receipts(&[receipt], l1_system_config_address, ecotone_active)
            .unwrap();

        assert_eq!(
            system_config.batcher_address,
            address!("000000000000000000000000000000000000bEEF"),
        );
    }

    #[test]
    fn test_system_config_update_batcher_log() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000000");

        let mut system_config = SystemConfig::default();

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the batcher address.
        system_config.process_config_update_log(&update_log, false).unwrap();

        assert_eq!(
            system_config.batcher_address,
            address!("000000000000000000000000000000000000bEEF")
        );
    }

    #[test]
    fn test_system_config_update_gas_config_log() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000001");

        let mut system_config = SystemConfig::default();

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000babe000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the batcher address.
        system_config.process_config_update_log(&update_log, false).unwrap();

        assert_eq!(system_config.overhead, U256::from(0xbabe));
        assert_eq!(system_config.scalar, U256::from(0xbeef));
    }

    #[test]
    fn test_system_config_update_gas_config_log_ecotone() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000001");

        let mut system_config = SystemConfig::default();

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000babe000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the gas limit.
        system_config.process_config_update_log(&update_log, true).unwrap();

        assert_eq!(system_config.overhead, U256::from(0));
        assert_eq!(system_config.scalar, U256::from(0xbeef));
    }

    #[test]
    fn test_system_config_update_gas_limit_log() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000002");

        let mut system_config = SystemConfig::default();

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the gas limit.
        system_config.process_config_update_log(&update_log, false).unwrap();

        assert_eq!(system_config.gas_limit, 0xbeef_u64);
    }

    #[test]
    fn test_system_config_update_eip1559_params_log() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000004");

        let mut system_config = SystemConfig::default();
        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000babe0000beef").into()
            )
        };

        // Update the EIP-1559 parameters.
        system_config.process_config_update_log(&update_log, false).unwrap();

        assert_eq!(system_config.eip1559_denominator, Some(0xbabe_u32));
        assert_eq!(system_config.eip1559_elasticity, Some(0xbeef_u32));
    }
}
