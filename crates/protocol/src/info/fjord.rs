//! Contains methods for the FJORD Hardfork.

use crate::utils::flz_compress_len;
use alloy_primitives::U256;

/// <https://github.com/ethereum-optimism/op-geth/blob/647c346e2bef36219cc7b47d76b1cb87e7ca29e4/core/types/rollup_cost.go#L79>
const L1_COST_FASTLZ_COEF: u64 = 836_500;

/// <https://github.com/ethereum-optimism/op-geth/blob/647c346e2bef36219cc7b47d76b1cb87e7ca29e4/core/types/rollup_cost.go#L78>
/// Inverted to be used with `saturating_sub`.
const L1_COST_INTERCEPT: u64 = 42_585_600;

/// <https://github.com/ethereum-optimism/op-geth/blob/647c346e2bef36219cc7b47d76b1cb87e7ca29e4/core/types/rollup_cost.go#82>
const MIN_TX_SIZE_SCALED: u64 = 100 * 1_000_000;

/// Calculate the estimated compressed transaction size in bytes, scaled by 1e6.
/// This value is computed based on the following formula:
/// max(minTransactionSize, intercept + fastlzCoef*fastlzSize)
pub fn estimate_fjord_tx_size(input: &[u8]) -> U256 {
    let fastlz_size = flz_compress_len(input) as u64;

    U256::from(
        fastlz_size
            .saturating_mul(L1_COST_FASTLZ_COEF)
            .saturating_sub(L1_COST_INTERCEPT)
            .max(MIN_TX_SIZE_SCALED),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_empty_tx_size() {
        let input = vec![];
        let expected = MIN_TX_SIZE_SCALED;
        assert_eq!(estimate_fjord_tx_size(&input), U256::from(expected));
    }

    #[test]
    fn test_estimate_small_tx_size() {
        let input = vec![13; 100];
        let expected = MIN_TX_SIZE_SCALED;
        assert_eq!(estimate_fjord_tx_size(&input), U256::from(expected));
    }

    #[test]
    fn test_estimate_large_tx_size() {
        let input = vec![13; 1_000_000];
        let fastlz_size = flz_compress_len(&input) as u64;
        let expected = fastlz_size * L1_COST_FASTLZ_COEF - L1_COST_INTERCEPT;
        assert_eq!(estimate_fjord_tx_size(&input), U256::from(expected));
    }
}
