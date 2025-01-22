#![cfg(feature = "jsonrpsee")]
//! The Optimism RPC API.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::net::IpAddr;

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{B256, U64};
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), allow(unused_imports))]
use getrandom as _; // required for compiling wasm32-unknown-unknown
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use maili_genesis::RollupConfig;
use maili_protocol::{ExecutingMessage, SafetyLevel};

use crate::{
    OutputResponse, PeerDump, PeerInfo, PeerStats, ProtocolVersion, SafeHeadResponse,
    SuperchainSignal, SyncStatus,
};

/// Optimism specified rpc interface.
///
/// https://docs.optimism.io/builders/node-operators/json-rpc
/// https://github.com/ethereum-optimism/optimism/blob/8dd17a7b114a7c25505cd2e15ce4e3d0f7e3f7c1/op-node/node/api.go#L114
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "optimism"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "optimism"))]
pub trait RollupNode {
    /// Get the output root at a specific block.
    #[method(name = "outputAtBlock")]
    async fn op_output_at_block(&self, block_number: BlockNumberOrTag)
        -> RpcResult<OutputResponse>;

    /// Gets the safe head at an L1 block height.
    #[method(name = "safeHeadAtL1Block")]
    async fn op_safe_head_at_l1_block(
        &self,
        block_number: BlockNumberOrTag,
    ) -> RpcResult<SafeHeadResponse>;

    /// Get the synchronization status.
    #[method(name = "syncStatus")]
    async fn op_sync_status(&self) -> RpcResult<SyncStatus>;

    /// Get the rollup configuration parameters.
    #[method(name = "rollupConfig")]
    async fn op_rollup_config(&self) -> RpcResult<RollupConfig>;

    /// Get the software version.
    #[method(name = "version")]
    async fn op_version(&self) -> RpcResult<String>;
}

/// The opp2p namespace handles peer interactions.
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "opp2p"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "opp2p"))]
pub trait OpP2PApi {
    /// Returns information of node
    #[method(name = "self")]
    async fn opp2p_self(&self) -> RpcResult<PeerInfo>;

    /// Returns information of peers
    #[method(name = "peers")]
    async fn opp2p_peers(&self) -> RpcResult<PeerDump>;

    /// Returns statistics of peers
    #[method(name = "peerStats")]
    async fn opp2p_peer_stats(&self) -> RpcResult<PeerStats>;

    /// Returns the discovery table
    #[method(name = "discoveryTable")]
    async fn opp2p_discovery_table(&self) -> RpcResult<Vec<String>>;

    /// Blocks the given peer
    #[method(name = "blockPeer")]
    async fn opp2p_block_peer(&self, peer: String) -> RpcResult<()>;

    /// Lists blocked peers
    #[method(name = "listBlockedPeers")]
    async fn opp2p_list_blocked_peers(&self) -> RpcResult<Vec<String>>;

    /// Blocks the given address
    #[method(name = "blocAddr")]
    async fn opp2p_block_addr(&self, ip: IpAddr) -> RpcResult<()>;

    /// Unblocks the given address
    #[method(name = "unblockAddr")]
    async fn opp2p_unblock_addr(&self, ip: IpAddr) -> RpcResult<()>;

    /// Lists blocked addresses
    #[method(name = "listBlockedAddrs")]
    async fn opp2p_list_blocked_addrs(&self) -> RpcResult<Vec<IpAddr>>;

    // TODO: should be IPNet?
    /// Blocks the given subnet
    #[method(name = "blockSubnet")]
    async fn opp2p_block_subnet(&self, subnet: String) -> RpcResult<()>;

    // TODO: should be IPNet?
    /// Unblocks the given subnet
    #[method(name = "unblockSubnet")]
    async fn opp2p_unblock_subnet(&self, subnet: String) -> RpcResult<()>;

    // TODO: should be IPNet?
    /// Lists blocked subnets
    #[method(name = "listBlockedSubnets")]
    async fn opp2p_list_blocked_subnets(&self) -> RpcResult<Vec<String>>;

    /// Protects the given peer
    #[method(name = "protectPeer")]
    async fn opp2p_protect_peer(&self, peer: String) -> RpcResult<()>;

    /// Unprotects the given peer
    #[method(name = "unprotectPeer")]
    async fn opp2p_unprotect_peer(&self, peer: String) -> RpcResult<()>;

    /// Connects to the given peer
    #[method(name = "connectPeer")]
    async fn opp2p_connect_peer(&self, peer: String) -> RpcResult<()>;

    /// Disconnects from the given peer
    #[method(name = "disconnectPeer")]
    async fn opp2p_disconnect_peer(&self, peer: String) -> RpcResult<()>;
}

/// The admin namespace endpoints
///
/// <https://github.com/ethereum-optimism/optimism/blob/c7ad0ebae5dca3bf8aa6f219367a95c15a15ae41/op-node/node/api.go#L28-L36>
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "admin"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "admin"))]
pub trait OpAdminApi {
    /// Resets the derivation pipeline.
    #[method(name = "resetDerivationPipeline")]
    async fn admin_reset_derivation_pipeline(&self) -> RpcResult<()>;

    /// Starts the sequencer.
    #[method(name = "startSequencer")]
    async fn admin_start_sequencer(&self, block_hash: B256) -> RpcResult<()>;

    /// Stops the sequencer.
    #[method(name = "stopSequencer")]
    async fn admin_stop_sequencer(&self) -> RpcResult<B256>;

    /// Returns the sequencer status.
    #[method(name = "sequencerActive")]
    async fn admin_sequencer_active(&self) -> RpcResult<bool>;
}

/// Engine API extension for Optimism superchain signaling
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "engine"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "engine"))]
pub trait EngineApiExt {
    /// Signal superchain v1 message
    ///
    /// The execution engine SHOULD warn when the recommended version is newer than the current
    /// version. The execution engine SHOULD take safety precautions if it does not meet
    /// the required version.
    ///
    /// # Returns
    /// The latest supported OP-Stack protocol version of the execution engine.
    ///
    /// See: <https://specs.optimism.io/protocol/exec-engine.html#engine_signalsuperchainv1>
    #[method(name = "signalSuperchainV1")]
    async fn signal_superchain_v1(&self, signal: SuperchainSignal) -> RpcResult<ProtocolVersion>;
}

/// Op API extension for controlling the miner.
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "miner"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "miner"))]
pub trait MinerApiExt {
    /// Sets the maximum data availability size of any tx allowed in a block, and the total max l1
    /// data size of the block. 0 means no maximum.
    #[method(name = "setMaxDASize")]
    async fn set_max_da_size(&self, max_tx_size: U64, max_block_size: U64) -> RpcResult<bool>;
}

/// Supervisor API for interop.
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "supervisor"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "supervisor"))]
pub trait SupervisorApi {
    /// Checks if the given messages meet the given minimum safety level.
    #[method(name = "checkMessages")]
    async fn check_messages(
        &self,
        messages: Vec<ExecutingMessage>,
        min_safety: SafetyLevel,
    ) -> RpcResult<()>;
}
