use alloy_network::Network;
use alloy_provider::Provider;
use alloy_transport::{Transport, TransportResult};
use maili_common::{ProtocolVersion, SuperchainSignal};

/// Extension trait that gives access to additional Optimism engine API RPC methods, that are not
/// available for L1.
///
/// Note:
/// > The provider should use a JWT authentication layer.
///
/// This follows the Optimism specs that can be found at:
/// <https://specs.optimism.io/protocol/exec-engine.html#engine-api>
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait EngineExtApi<N, T>: Send + Sync {
    /// Signals superchain information to the Engine
    ///
    /// V1 signals which protocol version is recommended and required.
    async fn signal_superchain_v1(
        &self,
        signal: SuperchainSignal,
    ) -> TransportResult<ProtocolVersion>;
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<N, T, P> EngineExtApi<N, T> for P
where
    N: Network,
    T: Transport + Clone,
    P: Provider<T, N>,
{
    async fn signal_superchain_v1(
        &self,
        signal: SuperchainSignal,
    ) -> TransportResult<ProtocolVersion> {
        self.client().request("engine_signalSuperchainV1", (signal,)).await
    }
}
