use core::time::Duration;

use crate::api::SupervisorApiClient;
use alloy_primitives::Log;
use maili_protocol::ExecutingMessage;

/// TODO docs.
#[derive(thiserror::Error, Debug)]
pub enum ExecutingMessageValidatorError {
    /// TODO docs.
    #[error("TODO")]
    Todo,
    /// TODO docs.
    #[error("TODO: {0}")]
    AlloySolTypesError(#[from] alloy_sol_types::Error),
}

/// TODO docs.
#[cfg(all(feature = "jsonrpsee", feature = "client"))]
pub trait ExecutingMessageValidator {
    /// TODO docs.
    type SupervisorClient: SupervisorApiClient;

    /// TODO docs.
    fn parse_messages(
        logs: &[Log],
    ) -> Result<Vec<ExecutingMessage>, ExecutingMessageValidatorError> {
        logs.iter()
            .map(|log| {
                // TODO: should we `impl From<Log> for ExecutingMessage`?
                // There is `impl From<Log> for MessagePayload` but I'm unsure
                // about the relationship between `MessagePayload` and `ExecutingMessage`
                ExecutingMessage::abi_decode(&log.data.data, true)
                    .map_err(ExecutingMessageValidatorError::AlloySolTypesError)
            })
            .collect()
    }

    /// TODO docs.
    fn validate_messages(
        supervisor: Self::SupervisorClient,
        messages: &[ExecutingMessage],
        timeout: Option<Duration>,
    ) -> Result<(), ExecutingMessageValidatorError> {
        todo!()
    }
}
