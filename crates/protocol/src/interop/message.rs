//! Message event primitives for OP stack interoperability.
//!
//! <https://specs.optimism.io/interop/messaging.html#messaging>
//! <https://github.com/ethereum-optimism/optimism/blob/34d5f66ade24bd1f3ce4ce7c0a6cfc1a6540eca1/packages/contracts-bedrock/src/L2/CrossL2Inbox.sol>

use crate::CROSS_L2_INBOX_ADDRESS;
use alloc::{vec, vec::Vec};
use alloy_consensus::TxReceipt;
use alloy_primitives::{keccak256, Bytes, Log};
use alloy_sol_types::{sol, SolEvent};
use derive_more::{AsRef, From};

sol! {
    /// @notice The struct for a pointer to a message payload in a remote (or local) chain.
    #[derive(Default, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    struct MessageIdentifier {
        address origin;
        uint256 blockNumber;
        uint256 logIndex;
        uint256 timestamp;
        #[cfg_attr(feature = "serde", serde(rename = "chainID"))]
        uint256 chainId;
    }

    /// @notice Emitted when a cross chain message is being executed.
    /// @param msgHash Hash of message payload being executed.
    /// @param id Encoded Identifier of the message.
    #[derive(Default, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    event ExecutingMessage(bytes32 indexed msgHash, MessageIdentifier id);

    /// @notice Executes a cross chain message on the destination chain.
    /// @param _id      Identifier of the message.
    /// @param _target  Target address to call.
    /// @param _message Message payload to call target with.
    function executeMessage(
        MessageIdentifier calldata _id,
        address _target,
        bytes calldata _message
    ) external;
}

/// A [MessagePayload] is the raw payload of an initiating message.
#[derive(Debug, Clone, From, AsRef, PartialEq, Eq)]
pub struct MessagePayload(Bytes);

impl From<&Log> for MessagePayload {
    fn from(log: &Log) -> Self {
        let mut data = vec![0u8; log.topics().len() * 32 + log.data.data.len()];
        for (i, topic) in log.topics().iter().enumerate() {
            data[i * 32..(i + 1) * 32].copy_from_slice(topic.as_ref());
        }
        data[(log.topics().len() * 32)..].copy_from_slice(log.data.data.as_ref());
        data.into()
    }
}

impl From<Vec<u8>> for MessagePayload {
    fn from(data: Vec<u8>) -> Self {
        Self(Bytes::from(data))
    }
}

impl From<executeMessageCall> for ExecutingMessage {
    fn from(call: executeMessageCall) -> Self {
        Self { id: call._id, msgHash: keccak256(call._message.as_ref()) }
    }
}

/// A wrapper type for [ExecutingMessage] containing the chain ID of the chain that the message was
/// executed on.
#[derive(Debug)]
pub struct EnrichedExecutingMessage {
    /// The inner [ExecutingMessage].
    pub inner: ExecutingMessage,
    /// The chain ID of the chain that the message was executed on.
    pub executing_chain_id: u64,
}

impl EnrichedExecutingMessage {
    /// Create a new [EnrichedExecutingMessage] from an [ExecutingMessage] and a chain ID.
    pub const fn new(inner: ExecutingMessage, executing_chain_id: u64) -> Self {
        Self { inner, executing_chain_id }
    }
}
/// Extracts all [ExecutingMessage] logs from a list of receipts.
pub fn extract_executing_messages(receipts: &[impl TxReceipt<Log = Log>]) -> Vec<ExecutingMessage> {
    receipts.iter().fold(Vec::new(), |mut acc, envelope| {
        let executing_messages = envelope.logs().iter().filter_map(|log| {
            (log.address == CROSS_L2_INBOX_ADDRESS && log.topics().len() == 2)
                .then(|| ExecutingMessage::decode_log_data(&log.data, true).ok())
                .flatten()
        });

        acc.extend(executing_messages);
        acc
    })
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use alloy_consensus::{Receipt, ReceiptEnvelope, ReceiptWithBloom};
    use alloy_primitives::{address, hex, keccak256, uint, LogData, B256, U256};
    use alloy_sol_types::SolValue;

    const MESSAGE: [u8; 4] = hex!("deadbeef");

    #[test]
    fn test_extract_executing_message_single() {
        let message_hash: B256 = keccak256(MESSAGE);
        let origin_address = address!("6887246668a3b87F54DeB3b94Ba47a6f63F32985");
        let origin_log_index = 100;
        let origin_timestamp = 100;
        let origin_chain_id = 100;
        let log = Log {
            address: CROSS_L2_INBOX_ADDRESS,
            data: LogData::new(
                vec![ExecutingMessage::SIGNATURE_HASH, message_hash],
                MessageIdentifier {
                    origin: origin_address,
                    blockNumber: U256::ZERO,
                    logIndex: U256::from(origin_log_index),
                    timestamp: U256::from(origin_timestamp),
                    chainId: U256::from(origin_chain_id),
                }
                .abi_encode()
                .into(),
            )
            .unwrap(),
        };
        assert_eq!(log.topics().len(), 2);
        let receipts = ReceiptEnvelope::Eip1559(ReceiptWithBloom {
            receipt: Receipt { logs: vec![log], ..Default::default() },
            ..Default::default()
        });
        let messages = extract_executing_messages(&[receipts]);
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_message_identifier_serde() {
        let raw_id = r#"
            {
                "origin": "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985",
                "blockNumber": 123456,
                "logIndex": 789,
                "timestamp": 1618932000,
                "chainID": 420
            }
        "#;
        let id: MessageIdentifier = serde_json::from_str(raw_id).unwrap();
        let expected = MessageIdentifier {
            origin: "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985".parse().unwrap(),
            blockNumber: uint!(123456_U256),
            logIndex: uint!(789_U256),
            timestamp: uint!(1618932000_U256),
            chainId: uint!(420_U256),
        };
        assert_eq!(id, expected);
    }
    #[test]
    fn test_executing_message_serde() {
        let raw_msg = r#"
        {
            "id": {
                "origin": "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985",
                "blockNumber": 123456,
                "logIndex": 789,
                "timestamp": 1618932000,
                "chainID": 420
            },
            "msgHash": "0xef8cc21bdbab8d2b60b054460768b1db67c8906b6a2bdf9bc287b3654326fc76"
        }
    "#;
        let msg: ExecutingMessage = serde_json::from_str(raw_msg).unwrap();
        let expected = ExecutingMessage {
            id: MessageIdentifier {
                origin: "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985".parse().unwrap(),
                blockNumber: uint!(123456_U256),
                logIndex: uint!(789_U256),
                timestamp: uint!(1618932000_U256),
                chainId: uint!(420_U256),
            },
            msgHash: "0xef8cc21bdbab8d2b60b054460768b1db67c8906b6a2bdf9bc287b3654326fc76"
                .parse()
                .unwrap(),
        };
        assert_eq!(msg, expected);
    }
}
