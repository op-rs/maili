//! Defines the supervisor API and Client.

use maili_interop::{ExecutingMessage, SafetyLevel};
use alloy_rpc_client::{ReqwestClient};

/// An interface for the `op-supervisor` component of the OP Stack.
///
/// <https://github.com/ethereum-optimism/optimism/blob/develop/op-supervisor/supervisor/frontend/frontend.go#L18-L28>
pub trait Supervisor {
    type Error;

    /// Returns if the messages meet the minimum safety level.
    async fn check_messages(&self, messages: &[ExecutingMessage], min_safety: SafetyLevel) -> Result<(), Error>;
}

/// An error from the `op-supervisor`.
#[derive(Copy, Clone, Debug, Display, From)]
pub enum SupervisorError {
}


pub SupervisorClient {
    /// The inner RPC client.
    client: ReqwestClient,
}

impl SupervisorClient {
    /// Creates a new `SupervisorClient` with the given `client`.
    pub fn new(client: ReqwestClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Supervisor for SupervisorClient {
    async fn check_messages(&self, messages: &[ExecutingMessage], min_safety: SafetyLevel) -> Result<(), SupervisorError> {
        self.client
            .request("supervisor_checkMessages", (messages, min_safety))
            .await
            .map_err(|_| SupervisorError::CheckMessages)
    }
}
