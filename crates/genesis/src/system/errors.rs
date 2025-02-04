//! Contains an error type for system config updates.

/// An error for processing the [`SystemConfig`] update log.
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SystemConfigUpdateError {
    /// An error occurred while processing the update log.
    #[error("Log processing error: {0}")]
    LogProcessing(LogProcessingError),
    /// A batcher update error.
    #[error("Batcher update error: {0}")]
    Batcher(BatcherUpdateError),
    /// A gas config update error.
    #[error("Gas config update error: {0}")]
    GasConfig(GasConfigUpdateError),
    /// A gas limit update error.
    #[error("Gas limit update error: {0}")]
    GasLimit(GasLimitUpdateError),
    /// An EIP-1559 parameter update error.
    #[error("EIP-1559 parameter update error: {0}")]
    Eip1559(EIP1559UpdateError),
    /// An operator fee parameter update error.
    #[error("Operator fee parameter update error: {0}")]
    OperatorFee(OperatorFeeUpdateError),
}
