use core::time::Duration;

use crate::api::SupervisorApiClient;
use alloy_primitives::Log;
use alloy_sol_types::SolEvent;
use maili_interop::ExecutingMessage;
use maili_interop::CROSS_L2_INBOX_ADDRESS;

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

    /// TODO docs.
    fn validate_messages(
        supervisor: Self::SupervisorClient,
        messages: &[ExecutingMessage],
        timeout: Option<Duration>,
    ) -> Result<(), ExecutingMessageValidatorError> {
        todo!()
    }
}
