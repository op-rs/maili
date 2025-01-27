use crate::api::SupervisorApiClient;
use alloy_primitives::Log;
use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use core::time::Duration;
use maili_interop::CROSS_L2_INBOX_ADDRESS;
use maili_interop::{ExecutingMessage, SafetyLevel};
use tokio::time::error::Elapsed;

/// Failures occurring during validation of [ExecutingMessage]s.
#[derive(thiserror::Error, Debug)]
pub enum ExecutingMessageValidatorError {
    /// Failure during Supervisor's validation of [ExecutingMessage]s.
    #[error("Supervisor determined messages are invalid: {0}")]
    SupervisorValidationError(#[from] jsonrpsee_core::ClientError),

    /// Message validation against the Supervisor took longer than allowed.
    #[error("Message validation timed out: {0}")]
    ValidationTimeout(#[from] Elapsed),
}

/// Interacts with a Supervisor to validate [ExecutingMessage]s.
#[async_trait]
#[cfg(all(feature = "jsonrpsee", feature = "client"))]
pub trait ExecutingMessageValidator {
    /// RPC client to Supervisor instance used for [ExecutingMessage] validation.
    type SupervisorClient: SupervisorApiClient + Sync;

    /// Default duration that message validation is not allowed to exceed.
    const DEFAULT_TIMEOUT: Duration;

    /// Extracts [ExecutingMessage]s from the [Log] if there are any.
    fn parse_messages(logs: &[Log]) -> impl Iterator<Item = ExecutingMessage> {
        logs.iter().filter_map(|log| {
            // TODO: Are there any error variants here that we want to consider
            // as failures rather than filtering out with `ok()`?
            (log.address == CROSS_L2_INBOX_ADDRESS && log.topics().len() == 2)
                .then(|| ExecutingMessage::decode_log_data(&log.data, true).ok())
                .flatten()
        })
    }

    /// Validates a list of [ExecutingMessages] against a Supervisor instance
    /// accessed through the [SupervisorClient].
    async fn validate_messages(
        supervisor: &Self::SupervisorClient,
        messages: &[ExecutingMessage],
        safety: SafetyLevel,
        timeout: Option<Duration>,
    ) -> Result<(), ExecutingMessageValidatorError> {
        // Set timeout duration based on input if provided.
        let timeout = match timeout {
            Some(t) => t,
            None => Self::DEFAULT_TIMEOUT,
        };

        // Construct the future to validate all messages using supervisor.
        // TODO: SupervisorApiClient::check_messages() should take &[ExecutingMessage]?
        let messages = messages.to_vec();
        let check = async {
            supervisor
                .check_messages(messages, safety)
                .await
                .map_err(|e| ExecutingMessageValidatorError::SupervisorValidationError(e))
        };

        // Await the validation future with timeout.
        tokio::time::timeout(timeout, check)
            .await
            .map_err(ExecutingMessageValidatorError::ValidationTimeout)?
    }
}
