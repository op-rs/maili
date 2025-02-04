// Contains the [`SystemConfigUpdateType`] type.

/// Represents type of update to the system config.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u64)]
pub enum SystemConfigUpdateType {
    /// Batcher update type
    Batcher = 0,
    /// Gas config update type
    GasConfig = 1,
    /// Gas limit update type
    GasLimit = 2,
    /// Unsafe block signer update type
    UnsafeBlockSigner = 3,
    /// EIP-1559 parameters update type
    Eip1559 = 4,
    /// Operator fee parameter update
    OperatorFee = 5,
}

impl TryFrom<u64> for SystemConfigUpdateType {
    type Error = SystemConfigUpdateError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Batcher),
            1 => Ok(Self::GasConfig),
            2 => Ok(Self::GasLimit),
            3 => Ok(Self::UnsafeBlockSigner),
            4 => Ok(Self::Eip1559),
            5 => Ok(Self::OperatorFee),
            _ => Err(SystemConfigUpdateError::LogProcessing(
                LogProcessingError::InvalidSystemConfigUpdateType(value),
            )),
        }
    }
}
