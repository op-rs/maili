//! Test utilities for `kona-interop`.

use crate::{
    ExecutingMessage, InteropProvider, InteropProviderResult, MessageIdentifier,
    CROSS_L2_INBOX_ADDRESS,
};
use alloc::{boxed::Box, vec, vec::Vec};
use alloy_consensus::{Header, Receipt, ReceiptEnvelope, ReceiptWithBloom, Sealed, TxReceipt};
use alloy_primitives::{map::HashMap, Address, Bytes, Log, LogData, B256};
use async_trait::async_trait;

/// A mock interop provider.
#[derive(Debug, Clone, Default)]
pub struct MockInteropProvider<T: TxReceipt<Log = Log>> {
    /// A map of chain IDs to a map of block numbers to sealed headers.
    pub headers: HashMap<u64, HashMap<u64, Sealed<Header>>>,
    /// A map of chain IDs to a map of block numbers to receipts.
    pub receipts: HashMap<u64, HashMap<u64, Vec<T>>>,
}

impl<T> MockInteropProvider<T>
where
    T: TxReceipt<Log = Log>,
{
    /// Construct a new mock interop provider.
    pub const fn new(
        headers: HashMap<u64, HashMap<u64, Sealed<Header>>>,
        receipts: HashMap<u64, HashMap<u64, Vec<T>>>,
    ) -> Self {
        Self { headers, receipts }
    }
}

#[async_trait]
impl<T> InteropProvider for MockInteropProvider<T>
where
    T: TxReceipt<Log = Log>,
{
    type Receipt = T;

    /// Fetch a [Header] by its number.
    async fn header_by_number(&self, chain_id: u64, number: u64) -> InteropProviderResult<Header> {
        Ok(self
            .headers
            .get(&chain_id)
            .and_then(|headers| headers.get(&number))
            .unwrap()
            .inner()
            .clone())
    }

    /// Fetch a [Header] by its hash.
    async fn header_by_hash(&self, chain_id: u64, hash: B256) -> InteropProviderResult<Header> {
        Ok(self
            .headers
            .get(&chain_id)
            .and_then(|headers| headers.values().find(|header| header.hash() == hash))
            .unwrap()
            .inner()
            .clone())
    }

    /// Fetch all receipts for a given block by number.
    async fn receipts_by_number(
        &self,
        chain_id: u64,
        number: u64,
    ) -> InteropProviderResult<Vec<T>> {
        Ok(self.receipts.get(&chain_id).and_then(|receipts| receipts.get(&number)).unwrap().clone())
    }

    /// Fetch all receipts for a given block by hash.
    async fn receipts_by_hash(
        &self,
        chain_id: u64,
        block_hash: B256,
    ) -> InteropProviderResult<Vec<T>> {
        Ok(self
            .receipts
            .get(&chain_id)
            .and_then(|receipts| {
                let headers = self.headers.get(&chain_id).unwrap();
                let number =
                    headers.values().find(|header| header.hash() == block_hash).unwrap().number;
                receipts.get(&number)
            })
            .unwrap()
            .clone())
    }
}

/// A builder for constructing a superchain.
#[derive(Debug)]
pub struct SuperchainBuilder<T: TxReceipt<Log = Log> + alloc::fmt::Debug> {
    /// Chains in the superchain
    chains: HashMap<u64, ChainBuilder<T>>,
    /// The current superchain timestamp.
    timestamp: u64,
}

/// A builder for a single chain in a superchain.
#[derive(Debug)]
pub struct ChainBuilder<T: TxReceipt<Log = Log> + alloc::fmt::Debug> {
    /// The header of the chain.
    header: Header,
    /// The receipts of the chain.
    receipts: Vec<T>,
}

impl SuperchainBuilder<ReceiptEnvelope> {
    /// Create a new superchain builder.
    pub fn new(timestamp: u64) -> Self {
        Self { chains: Default::default(), timestamp }
    }

    /// Constructs a [`ChainBuilder`] for the given chain ID.
    pub fn chain(&mut self, chain_id: u64) -> &mut ChainBuilder<ReceiptEnvelope> {
        self.chains.entry(chain_id).or_insert_with(|| ChainBuilder::new(self.timestamp))
    }

    /// Builds the scenario into the format needed for testing
    pub fn build(self) -> (Vec<(u64, Sealed<Header>)>, MockInteropProvider<ReceiptEnvelope>) {
        let mut headers_map = HashMap::default();
        let mut receipts_map = HashMap::default();
        let mut sealed_headers = Vec::new();

        for (chain_id, chain) in self.chains {
            let header = chain.header;
            let header_hash = header.hash_slow();
            let sealed_header = header.seal(header_hash);

            let mut chain_headers = HashMap::default();
            chain_headers.insert(0, sealed_header.clone());
            headers_map.insert(chain_id, chain_headers);

            let mut chain_receipts = HashMap::default();
            chain_receipts.insert(0, chain.receipts);
            receipts_map.insert(chain_id, chain_receipts);

            sealed_headers.push((chain_id, sealed_header));
        }

        (sealed_headers, MockInteropProvider::new(headers_map, receipts_map))
    }
}

impl ChainBuilder<ReceiptEnvelope> {
    /// Create a new chain builder.
    pub fn new(timestamp: u64) -> Self {
        Self { header: Header { timestamp, ..Default::default() }, receipts: Vec::new() }
    }

    /// Adds and initiating message to the chain.
    pub fn add_initiating_message(&mut self, message_data: Bytes) -> &mut Self {
        let receipt = ReceiptEnvelope::Eip1559(ReceiptWithBloom {
            receipt: Receipt {
                logs: vec![Log {
                    address: Address::ZERO,
                    data: LogData::new(vec![], message_data).unwrap(),
                }],
                ..Default::default()
            },
            ..Default::default()
        });
        self.receipts.push(receipt);
        self
    }

    /// Adds an executing message to the chain.
    pub fn add_executing_message(
        &mut self,
        message_hash: B256,
        origin_log_index: u64,
        origin_chain_id: u64,
        origin_timestamp: u64,
    ) -> &mut Self {
        self.add_executing_message_with_origin(
            message_hash,
            Address::ZERO,
            origin_log_index,
            origin_chain_id,
            origin_timestamp,
        )
    }

    /// Adds an executing message to the chain with an origin.
    pub fn add_executing_message_with_origin(
        &mut self,
        message_hash: B256,
        origin_address: Address,
        origin_log_index: u64,
        origin_chain_id: u64,
        origin_timestamp: u64,
    ) -> &mut Self {
        let receipt = ReceiptEnvelope::Eip1559(ReceiptWithBloom {
            receipt: Receipt {
                logs: vec![Log {
                    address: CROSS_L2_INBOX_ADDRESS,
                    data: LogData::new(
                        vec![ExecutingMessage::SIGNATURE_HASH, message_hash],
                        MessageIdentifier {
                            origin: origin_address,
                            block_number: 0,
                            log_index: origin_log_index,
                            timestamp: origin_timestamp,
                            chain_id: origin_chain_id,
                        }
                        .abi_encode()
                        .into(),
                    )
                    .unwrap(),
                }],
                ..Default::default()
            },
            ..Default::default()
        });
        self.receipts.push(receipt);
        self
    }
}
