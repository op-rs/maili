//! Traits for working with protocol types.

use alloc::{boxed::Box, string::ToString};
use alloy_consensus::Block;
use alloy_eips::eip2718::Encodable2718;
use async_trait::async_trait;
use core::fmt::Display;
use maili_common::OpTransaction;

use crate::L2BlockInfo;

/// Describes the functionality of a data source that fetches safe blocks.
#[async_trait]
pub trait BatchValidationProvider {
    /// The error type for the [BatchValidationProvider].
    type Error: Display + ToString;

    /// Signed (except for deposit) transaction.
    type Transaction: OpTransaction + Encodable2718;

    /// Returns the [L2BlockInfo] given a block number.
    ///
    /// Errors if the block does not exist.
    async fn l2_block_info_by_number(&mut self, number: u64) -> Result<L2BlockInfo, Self::Error>;

    /// Returns the OP [Block] for a given number.
    ///
    /// Errors if no block is available for the given block number.
    async fn block_by_number(
        &mut self,
        number: u64,
    ) -> Result<Block<Self::Transaction>, Self::Error>;
}
