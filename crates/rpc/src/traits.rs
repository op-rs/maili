use core::time::Duration;

use crate::api::SupervisorApiClient;
use alloy_consensus::Receipt;
use maili_protocol::ExecutingMessage;

/// TODO docs.
#[derive(thiserror::Error, Debug)]
pub enum ExecutingMessageValidatorError {
    /// TODO docs.
    #[error("TODO")]
    Todo,
}

/// TODO docs.
#[cfg(all(feature = "jsonrpsee", feature = "client"))]
pub trait ExecutingMessageValidator {
    /// TODO docs.
    type SupervisorClient: SupervisorApiClient;

    /// TODO docs.
    fn parse_messages(
        receipt: &Receipt,
    ) -> Result<Vec<ExecutingMessage>, ExecutingMessageValidatorError>;

    /// TODO docs.
    fn validate_messages(
        supervisor: Self::SupervisorClient,
        messages: &[ExecutingMessage],
        timeout: Option<Duration>,
    ) -> Result<(), ExecutingMessageValidatorError>;
}
