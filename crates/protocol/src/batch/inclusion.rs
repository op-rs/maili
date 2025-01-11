//! Module containing the [BatchWithInclusionBlock] struct.

use crate::{Batch, BatchValidationProvider, BatchValidity, BlockInfo, L2BlockInfo};
use alloy_eips::eip2718::Encodable2718;
use maili_common::OpTransaction;
use op_alloy_genesis::RollupConfig;

/// A batch with its inclusion block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchWithInclusionBlock {
    /// The inclusion block
    pub inclusion_block: BlockInfo,
    /// The batch
    pub batch: Batch,
}

impl BatchWithInclusionBlock {
    /// Creates a new batch with inclusion block.
    pub const fn new(inclusion_block: BlockInfo, batch: Batch) -> Self {
        Self { inclusion_block, batch }
    }

    /// Validates the batch can be applied on top of the specified L2 safe head.
    /// The first entry of the l1_blocks should match the origin of the l2_safe_head.
    /// One or more consecutive l1_blocks should be provided.
    /// In case of only a single L1 block, the decision whether a batch is valid may have to stay
    /// undecided.
    pub async fn check_batch<BF>(
        &self,
        cfg: &RollupConfig,
        l1_blocks: &[BlockInfo],
        l2_safe_head: L2BlockInfo,
        fetcher: &mut BF,
    ) -> BatchValidity
    where
        BF: BatchValidationProvider<Transaction: OpTransaction + Encodable2718>,
    {
        match &self.batch {
            Batch::Single(single_batch) => {
                single_batch.check_batch(cfg, l1_blocks, l2_safe_head, &self.inclusion_block)
            }
            Batch::Span(span_batch) => {
                span_batch
                    .check_batch(cfg, l1_blocks, l2_safe_head, &self.inclusion_block, fetcher)
                    .await
            }
        }
    }
}
