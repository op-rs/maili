//! Contains the [`SystemConfig`] type definition.

use alloy_consensus::{Eip658Value, Receipt};
use alloy_primitives::{b256, Address, Log, B256, B64, U256, U64};
use alloy_sol_types::{sol, SolType};

/// System configuration.
#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SystemConfig {
    /// Batcher address
    #[cfg_attr(feature = "serde", serde(rename = "batcherAddress", alias = "batcherAddr"))]
    pub batcher_address: Address,
    /// Fee overhead value
    pub overhead: U256,
    /// Fee scalar value
    pub scalar: U256,
    /// Gas limit value
    pub gas_limit: u64,
    /// Base fee scalar value
    pub base_fee_scalar: Option<u64>,
    /// Blob base fee scalar value
    pub blob_base_fee_scalar: Option<u64>,
    /// EIP-1559 denominator
    pub eip1559_denominator: Option<u32>,
    /// EIP-1559 elasticity
    pub eip1559_elasticity: Option<u32>,
    /// The operator fee scalar (isthmus hardfork)
    pub operator_fee_scalar: Option<u32>,
    /// The operator fee constant (isthmus hardfork)
    pub operator_fee_constant: Option<u64>,
}

impl SystemConfig {
    /// Filters all L1 receipts to find config updates and applies the config updates.
    pub fn update_with_receipts(
        &mut self,
        receipts: &[Receipt],
        l1_system_config_address: Address,
        ecotone_active: bool,
    ) -> Result<(), SystemConfigUpdateError> {
        for receipt in receipts {
            if Eip658Value::Eip658(false) == receipt.status {
                continue;
            }

            receipt.logs.iter().try_for_each(|log| {
                let topics = log.topics();
                if log.address == l1_system_config_address
                    && !topics.is_empty()
                    && topics[0] == CONFIG_UPDATE_TOPIC
                {
                    // Safety: Error is bubbled up by the trailing `?`
                    self.process_config_update_log(log, ecotone_active)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }

    /// Returns the eip1559 parameters from a [SystemConfig] encoded as a [B64].
    pub fn eip_1559_params(
        &self,
        rollup_config: &RollupConfig,
        parent_timestamp: u64,
        next_timestamp: u64,
    ) -> Option<B64> {
        let is_holocene = rollup_config.is_holocene_active(next_timestamp);

        // For the first holocene block, a zero'd out B64 is returned to signal the
        // execution layer to use the canyon base fee parameters. Else, the system
        // config's eip1559 parameters are encoded as a B64.
        if is_holocene && !rollup_config.is_holocene_active(parent_timestamp) {
            Some(B64::ZERO)
        } else {
            is_holocene.then_some(B64::from_slice(
                &[
                    self.eip1559_denominator.unwrap_or_default().to_be_bytes(),
                    self.eip1559_elasticity.unwrap_or_default().to_be_bytes(),
                ]
                .concat(),
            ))
        }
    }

    /// Decodes an EVM log entry emitted by the system config contract and applies it as a
    /// [SystemConfig] change.
    ///
    /// Parse log data for:
    ///
    /// ```text
    /// event ConfigUpdate(
    ///    uint256 indexed version,
    ///    UpdateType indexed updateType,
    ///    bytes data
    /// );
    /// ```
    fn process_config_update_log(
        &mut self,
        log: &Log,
        ecotone_active: bool,
    ) -> Result<SystemConfigUpdateType, SystemConfigUpdateError> {
        // Validate the log
        if log.topics().len() < 3 {
            return Err(SystemConfigUpdateError::LogProcessing(
                LogProcessingError::InvalidTopicLen(log.topics().len()),
            ));
        }
        if log.topics()[0] != CONFIG_UPDATE_TOPIC {
            return Err(SystemConfigUpdateError::LogProcessing(LogProcessingError::InvalidTopic));
        }

        // Parse the config update log
        let version = log.topics()[1];
        if version != CONFIG_UPDATE_EVENT_VERSION_0 {
            return Err(SystemConfigUpdateError::LogProcessing(
                LogProcessingError::UnsupportedVersion(version),
            ));
        }
        let Ok(topic_bytes) = <&[u8; 8]>::try_from(&log.topics()[2].as_slice()[24..]) else {
            return Err(SystemConfigUpdateError::LogProcessing(
                LogProcessingError::UpdateTypeDecodingError,
            ));
        };
        let update_type = u64::from_be_bytes(*topic_bytes);
        let log_data = log.data.data.as_ref();

        // Apply the update
        match update_type.try_into()? {
            SystemConfigUpdateType::Batcher => {
                self.update_batcher_address(log_data).map_err(SystemConfigUpdateError::Batcher)
            }
            SystemConfigUpdateType::GasConfig => self
                .update_gas_config(log_data, ecotone_active)
                .map_err(SystemConfigUpdateError::GasConfig),
            SystemConfigUpdateType::GasLimit => {
                self.update_gas_limit(log_data).map_err(SystemConfigUpdateError::GasLimit)
            }
            SystemConfigUpdateType::Eip1559 => {
                self.update_eip1559_params(log_data).map_err(SystemConfigUpdateError::Eip1559)
            }
            SystemConfigUpdateType::OperatorFee => self
                .update_operator_fee_params(log_data)
                .map_err(SystemConfigUpdateError::OperatorFee),
            // Ignored in derivation
            SystemConfigUpdateType::UnsafeBlockSigner => {
                Ok(SystemConfigUpdateType::UnsafeBlockSigner)
            }
        }
    }

    /// Updates the batcher address in the [SystemConfig] given the log data.
    fn update_batcher_address(
        &mut self,
        log_data: &[u8],
    ) -> Result<SystemConfigUpdateType, BatcherUpdateError> {
        if log_data.len() != 96 {
            return Err(BatcherUpdateError::InvalidDataLen(log_data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
            return Err(BatcherUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(BatcherUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
            return Err(BatcherUpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(BatcherUpdateError::InvalidDataLength(length));
        }

        let Ok(batcher_address) = <sol!(address)>::abi_decode(&log_data[64..], true) else {
            return Err(BatcherUpdateError::BatcherAddressDecodingError);
        };
        self.batcher_address = batcher_address;
        Ok(SystemConfigUpdateType::Batcher)
    }

    /// Updates the [SystemConfig] gas config - both the overhead and scalar values
    /// given the log data and rollup config.
    fn update_gas_config(
        &mut self,
        log_data: &[u8],
        ecotone_active: bool,
    ) -> Result<SystemConfigUpdateType, GasConfigUpdateError> {
        if log_data.len() != 128 {
            return Err(GasConfigUpdateError::InvalidDataLen(log_data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
            return Err(GasConfigUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(GasConfigUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
            return Err(GasConfigUpdateError::LengthDecodingError);
        };
        if length != 64 {
            return Err(GasConfigUpdateError::InvalidDataLength(length));
        }

        let Ok(overhead) = <sol!(uint256)>::abi_decode(&log_data[64..96], true) else {
            return Err(GasConfigUpdateError::OverheadDecodingError);
        };
        let Ok(scalar) = <sol!(uint256)>::abi_decode(&log_data[96..], true) else {
            return Err(GasConfigUpdateError::ScalarDecodingError);
        };

        if ecotone_active
            && RollupConfig::check_ecotone_l1_system_config_scalar(scalar.to_be_bytes()).is_err()
        {
            // ignore invalid scalars, retain the old system-config scalar
            return Ok(SystemConfigUpdateType::GasConfig);
        }

        // Retain the scalar data in encoded form.
        self.scalar = scalar;

        // If ecotone is active, set the overhead to zero, otherwise set to the decoded value.
        self.overhead = if ecotone_active { U256::ZERO } else { overhead };

        Ok(SystemConfigUpdateType::GasConfig)
    }

    /// Updates the gas limit of the [SystemConfig] given the log data.
    fn update_gas_limit(
        &mut self,
        log_data: &[u8],
    ) -> Result<SystemConfigUpdateType, GasLimitUpdateError> {
        if log_data.len() != 96 {
            return Err(GasLimitUpdateError::InvalidDataLen(log_data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
            return Err(GasLimitUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(GasLimitUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
            return Err(GasLimitUpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(GasLimitUpdateError::InvalidDataLength(length));
        }

        let Ok(gas_limit) = <sol!(uint256)>::abi_decode(&log_data[64..], true) else {
            return Err(GasLimitUpdateError::GasLimitDecodingError);
        };
        self.gas_limit = U64::from(gas_limit).saturating_to::<u64>();
        Ok(SystemConfigUpdateType::GasLimit)
    }

    /// Updates the EIP-1559 parameters of the [SystemConfig] given the log data.
    fn update_eip1559_params(
        &mut self,
        log_data: &[u8],
    ) -> Result<SystemConfigUpdateType, EIP1559UpdateError> {
        if log_data.len() != 96 {
            return Err(EIP1559UpdateError::InvalidDataLen(log_data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
            return Err(EIP1559UpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(EIP1559UpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
            return Err(EIP1559UpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(EIP1559UpdateError::InvalidDataLength(length));
        }

        let Ok(eip1559_params) = <sol!(uint64)>::abi_decode(&log_data[64..], true) else {
            return Err(EIP1559UpdateError::EIP1559DecodingError);
        };

        self.eip1559_denominator = Some((eip1559_params >> 32) as u32);
        self.eip1559_elasticity = Some(eip1559_params as u32);

        Ok(SystemConfigUpdateType::Eip1559)
    }

    /// Updates the operator fee parameters of the [SystemConfig] given the log data.
    pub fn update_operator_fee_params(
        &mut self,
        log_data: &[u8],
    ) -> Result<SystemConfigUpdateType, OperatorFeeUpdateError> {
        if log_data.len() != 128 {
            return Err(OperatorFeeUpdateError::InvalidDataLen(log_data.len()));
        }

        let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
            return Err(OperatorFeeUpdateError::PointerDecodingError);
        };
        if pointer != 32 {
            return Err(OperatorFeeUpdateError::InvalidDataPointer(pointer));
        }
        let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
            return Err(OperatorFeeUpdateError::LengthDecodingError);
        };
        if length != 32 {
            return Err(OperatorFeeUpdateError::InvalidDataLength(length));
        }

        let Ok(operator_fee_scalar) = <sol!(uint32)>::abi_decode(&log_data[64..96], true) else {
            return Err(OperatorFeeUpdateError::ScalarDecodingError);
        };
        let Ok(operator_fee_constant) = <sol!(uint64)>::abi_decode(&log_data[96..], true) else {
            return Err(OperatorFeeUpdateError::ConstantDecodingError);
        };

        self.operator_fee_scalar = Some(operator_fee_scalar);
        self.operator_fee_constant = Some(operator_fee_constant);

        Ok(SystemConfigUpdateType::OperatorFee)
    }
}
