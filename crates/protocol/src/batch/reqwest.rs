//! Implementation of the `BatchValidationProvider` trait for the alloy `ReqwestProvider`.

use alloc::boxed::Box;
use alloy_consensus::{Block, BlockBody};
use alloy_network::{
    primitives::{BlockTransactions, BlockTransactionsKind, HeaderResponse},
    BlockResponse,
};
use alloy_provider::{Provider, ReqwestProvider};
use async_trait::async_trait;
use op_alloy_consensus::OpBlock;

use crate::{BatchValidationProvider, BlockInfo, L2BlockInfo};

/// An error encountered during batch validation provider operations.
#[derive(Debug, thiserror::Error)]
pub enum ReqwestBatchValidationProviderError {
    /// Requested block does not exist.
    #[error("Block not found: {0}")]
    BlockNotFound(u64),
    /// Processed block contains invalid or unexpected data.
    #[error("Invalid block: {0}. Error: {1}")]
    InvalidBlock(u64, String),
    /// An error from alloy transport layer.
    #[error(transparent)]
    TransportError(#[from] alloy_transport::TransportError),
}

#[async_trait]
impl BatchValidationProvider for ReqwestProvider {
    type Error = ReqwestBatchValidationProviderError;

    async fn l2_block_info_by_number(&mut self, number: u64) -> Result<L2BlockInfo, Self::Error> {
        // Get the block from the provider.
        let block = self
            .get_block_by_number(number.into(), BlockTransactionsKind::Full)
            .await?
            .ok_or(ReqwestBatchValidationProviderError::BlockNotFound(number))?;

        let header = block.header().as_ref();
        // Create the L2 block info.
        let block_info = BlockInfo::new(
            block.header().hash(),
            header.number,
            header.parent_hash,
            header.timestamp,
        );
        let l1_origin = (header.number, block.header().hash()); // TODO: We don't have valid values
                                                                // for this or access to genesis for
                                                                // L2BlockInfo::from_block_and_genesis().
        let seq_num = 0u64; // TODO: We don't have this value either.
        let l2_block_info = L2BlockInfo::new(block_info, l1_origin.into(), seq_num);
        Ok(l2_block_info)
    }

    async fn block_by_number(&mut self, number: u64) -> Result<OpBlock, Self::Error> {
        // Get the block from the provider.
        let block = self
            .get_block_by_number(number.into(), BlockTransactionsKind::Full)
            .await?
            .ok_or(ReqwestBatchValidationProviderError::BlockNotFound(number))?;

        // Convert the transactions.
        let transactions: Vec<_> = match block.transactions() {
            // TODO: avoid clone?
            BlockTransactions::Full(transactions) => {
                Ok(transactions.into_iter().map(|t| t.inner).collect())
            }
            _ => Err(ReqwestBatchValidationProviderError::InvalidBlock(
                number,
                "Expected Full block".to_string(),
            )),
        }?;

        // Create the block.
        Ok(Block {
            header: block.header().as_ref().clone(),
            body: BlockBody {
                transactions: transactions.to_vec(),
                // TODO: fill
                ommers: Default::default(),
                withdrawals: Default::default(),
            },
        })
    }
}
