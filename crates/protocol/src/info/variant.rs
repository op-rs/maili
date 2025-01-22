//! Contains the `L1BlockInfoTx` enum, containing different variants of the L1 block info
//! transaction.

use alloc::{format, string::ToString};
use alloy_consensus::Header;
use alloy_eips::{eip7840::BlobParams, BlockNumHash};
use alloy_primitives::{address, Address, Bytes, Sealable, Sealed, TxKind, B256, U256};
use maili_genesis::{RollupConfig, SystemConfig};
use op_alloy_consensus::{DepositSourceDomain, L1InfoDepositSource, TxDeposit};

use crate::{BlockInfoError, DecodeError, L1BlockInfoBedrock, L1BlockInfoEcotone};

use super::L1BlockInfoInterop;

/// The system transaction gas limit post-Regolith
const REGOLITH_SYSTEM_TX_GAS: u64 = 1_000_000;

/// The address of the L1 Block contract
pub(crate) const L1_BLOCK_ADDRESS: Address = address!("4200000000000000000000000000000000000015");

/// The depositor address of the L1 info transaction
pub(crate) const L1_INFO_DEPOSITOR_ADDRESS: Address =
    address!("deaddeaddeaddeaddeaddeaddeaddeaddead0001");

/// The [L1BlockInfoTx] enum contains variants for the different versions of the L1 block info
/// transaction on OP Stack chains.
///
/// This transaction always sits at the top of the block, and alters the `L1 Block` contract's
/// knowledge of the L1 chain.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum L1BlockInfoTx {
    /// A Bedrock L1 info transaction
    Bedrock(L1BlockInfoBedrock),
    /// An Ecotone L1 info transaction
    Ecotone(L1BlockInfoEcotone),
    /// An Interop L1 info transaction
    Interop(L1BlockInfoInterop),
}

impl L1BlockInfoTx {
    /// Creates a new [L1BlockInfoTx] from the given information.
    pub fn try_new(
        rollup_config: &RollupConfig,
        system_config: &SystemConfig,
        sequence_number: u64,
        l1_header: &Header,
        l2_block_time: u64,
    ) -> Result<Self, BlockInfoError> {
        // In the first block of Ecotone, the L1Block contract has not been upgraded yet due to the
        // upgrade transactions being placed after the L1 info transaction. Because of this,
        // for the first block of Ecotone, we send a Bedrock style L1 block info transaction
        let is_first_ecotone_block =
            rollup_config.ecotone_time.unwrap_or_default() == l2_block_time;

        // If ecotone is *not* active or this is the first block of ecotone, use Bedrock block info.
        if !rollup_config.is_ecotone_active(l2_block_time) || is_first_ecotone_block {
            return Ok(Self::Bedrock(L1BlockInfoBedrock {
                number: l1_header.number,
                time: l1_header.timestamp,
                base_fee: l1_header.base_fee_per_gas.unwrap_or(0),
                block_hash: l1_header.hash_slow(),
                sequence_number,
                batcher_address: system_config.batcher_address,
                l1_fee_overhead: system_config.overhead,
                l1_fee_scalar: system_config.scalar,
            }));
        }

        // --- Post-Ecotone Operations ---

        let scalar = system_config.scalar.to_be_bytes::<32>();
        let blob_base_fee_scalar = (scalar[0] == L1BlockInfoEcotone::L1_SCALAR)
            .then(|| {
                Ok::<u32, BlockInfoError>(u32::from_be_bytes(
                    scalar[24..28].try_into().map_err(|_| BlockInfoError::L1BlobBaseFeeScalar)?,
                ))
            })
            .transpose()?
            .unwrap_or_default();
        let base_fee_scalar = u32::from_be_bytes(
            scalar[28..32].try_into().map_err(|_| BlockInfoError::BaseFeeScalar)?,
        );

        if rollup_config.is_interop_active(l2_block_time)
            && rollup_config.interop_time.unwrap_or_default() != l2_block_time
        {
            Ok(Self::Interop(L1BlockInfoInterop {
                number: l1_header.number,
                time: l1_header.timestamp,
                base_fee: l1_header.base_fee_per_gas.unwrap_or(0),
                block_hash: l1_header.hash_slow(),
                sequence_number,
                batcher_address: system_config.batcher_address,
                blob_base_fee: l1_header.blob_fee(BlobParams::cancun()).unwrap_or(1),
                blob_base_fee_scalar,
                base_fee_scalar,
            }))
        } else {
            Ok(Self::Ecotone(L1BlockInfoEcotone {
                number: l1_header.number,
                time: l1_header.timestamp,
                base_fee: l1_header.base_fee_per_gas.unwrap_or(0),
                block_hash: l1_header.hash_slow(),
                sequence_number,
                batcher_address: system_config.batcher_address,
                blob_base_fee: l1_header.blob_fee(BlobParams::cancun()).unwrap_or(1),
                blob_base_fee_scalar,
                base_fee_scalar,
                empty_scalars: false,
                l1_fee_overhead: U256::ZERO,
            }))
        }
    }

    /// Creates a new [L1BlockInfoTx] from the given information and returns a typed [TxDeposit] to
    /// include at the top of a block.
    pub fn try_new_with_deposit_tx(
        rollup_config: &RollupConfig,
        system_config: &SystemConfig,
        sequence_number: u64,
        l1_header: &Header,
        l2_block_time: u64,
    ) -> Result<(Self, Sealed<TxDeposit>), BlockInfoError> {
        let l1_info =
            Self::try_new(rollup_config, system_config, sequence_number, l1_header, l2_block_time)?;

        let source = DepositSourceDomain::L1Info(L1InfoDepositSource {
            l1_block_hash: l1_info.block_hash(),
            seq_number: sequence_number,
        });

        let mut deposit_tx = TxDeposit {
            source_hash: source.source_hash(),
            from: L1_INFO_DEPOSITOR_ADDRESS,
            to: TxKind::Call(L1_BLOCK_ADDRESS),
            mint: None,
            value: U256::ZERO,
            gas_limit: 150_000_000,
            is_system_transaction: true,
            input: l1_info.encode_calldata(),
        };

        // With the regolith hardfork, system transactions were deprecated, and we allocate
        // a constant amount of gas for special transactions like L1 block info.
        if rollup_config.is_regolith_active(l2_block_time) {
            deposit_tx.is_system_transaction = false;
            deposit_tx.gas_limit = REGOLITH_SYSTEM_TX_GAS;
        }

        Ok((l1_info, deposit_tx.seal_slow()))
    }

    /// Decodes the [L1BlockInfoEcotone] object from ethereum transaction calldata.
    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> {
        let selector = r
            .get(0..4)
            .ok_or(DecodeError::ParseError("Slice out of range".to_string()))
            .and_then(|slice| {
                slice.try_into().map_err(|_| {
                    DecodeError::ParseError("Failed to convert 4byte slice to array".to_string())
                })
            })?;
        match selector {
            L1BlockInfoBedrock::L1_INFO_TX_SELECTOR => L1BlockInfoBedrock::decode_calldata(r)
                .map(Self::Bedrock)
                .map_err(|e| DecodeError::ParseError(format!("Bedrock decode error: {}", e))),
            L1BlockInfoEcotone::L1_INFO_TX_SELECTOR => L1BlockInfoEcotone::decode_calldata(r)
                .map(Self::Ecotone)
                .map_err(|e| DecodeError::ParseError(format!("Ecotone decode error: {}", e))),
            L1BlockInfoInterop::L1_INFO_TX_SELECTOR => L1BlockInfoInterop::decode_calldata(r)
                .map(Self::Interop)
                .map_err(|e| DecodeError::ParseError(format!("Interop decode error: {}", e))),
            _ => Err(DecodeError::InvalidSelector),
        }
    }

    /// Returns whether the scalars are empty.
    pub const fn empty_scalars(&self) -> bool {
        match self {
            Self::Bedrock(_) | Self::Interop(_) => false,
            Self::Ecotone(L1BlockInfoEcotone { empty_scalars, .. }) => *empty_scalars,
        }
    }

    /// Returns the block hash for the [L1BlockInfoTx].
    pub const fn block_hash(&self) -> B256 {
        match self {
            Self::Bedrock(ref tx) => tx.block_hash,
            Self::Ecotone(ref tx) => tx.block_hash,
            Self::Interop(ref tx) => tx.block_hash,
        }
    }

    /// Encodes the [L1BlockInfoTx] object into Ethereum transaction calldata.
    pub fn encode_calldata(&self) -> Bytes {
        match self {
            Self::Bedrock(bedrock_tx) => bedrock_tx.encode_calldata(),
            Self::Ecotone(ecotone_tx) => ecotone_tx.encode_calldata(),
            Self::Interop(interop_tx) => interop_tx.encode_calldata(),
        }
    }

    /// Returns the L1 [BlockNumHash] for the info transaction.
    pub const fn id(&self) -> BlockNumHash {
        match self {
            Self::Ecotone(L1BlockInfoEcotone { number, block_hash, .. })
            | Self::Bedrock(L1BlockInfoBedrock { number, block_hash, .. })
            | Self::Interop(L1BlockInfoInterop { number, block_hash, .. }) => {
                BlockNumHash { number: *number, hash: *block_hash }
            }
        }
    }

    /// Returns the l1 base fee.
    pub fn l1_base_fee(&self) -> U256 {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { base_fee, .. })
            | Self::Ecotone(L1BlockInfoEcotone { base_fee, .. })
            | Self::Interop(L1BlockInfoInterop { base_fee, .. }) => U256::from(*base_fee),
        }
    }

    /// Returns the l1 fee scalar.
    pub fn l1_fee_scalar(&self) -> U256 {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { l1_fee_scalar, .. }) => *l1_fee_scalar,
            Self::Ecotone(L1BlockInfoEcotone { base_fee_scalar, .. })
            | Self::Interop(L1BlockInfoInterop { base_fee_scalar, .. }) => {
                U256::from(*base_fee_scalar)
            }
        }
    }

    /// Returns the blob base fee.
    pub fn blob_base_fee(&self) -> U256 {
        match self {
            Self::Bedrock(_) => U256::ZERO,
            Self::Ecotone(L1BlockInfoEcotone { blob_base_fee, .. })
            | Self::Interop(L1BlockInfoInterop { blob_base_fee, .. }) => U256::from(*blob_base_fee),
        }
    }

    /// Returns the blob base fee scalar.
    pub fn blob_base_fee_scalar(&self) -> U256 {
        match self {
            Self::Bedrock(_) => U256::ZERO,
            Self::Ecotone(L1BlockInfoEcotone { blob_base_fee_scalar, .. })
            | Self::Interop(L1BlockInfoInterop { blob_base_fee_scalar, .. }) => {
                U256::from(*blob_base_fee_scalar)
            }
        }
    }

    /// Returns the L1 fee overhead for the info transaction. After ecotone, this value is ignored.
    pub const fn l1_fee_overhead(&self) -> U256 {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { l1_fee_overhead, .. }) => *l1_fee_overhead,
            Self::Ecotone(L1BlockInfoEcotone { l1_fee_overhead, .. }) => *l1_fee_overhead,
            Self::Interop(_) => U256::ZERO,
        }
    }

    /// Returns the batcher address for the info transaction
    pub const fn batcher_address(&self) -> Address {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { batcher_address, .. })
            | Self::Ecotone(L1BlockInfoEcotone { batcher_address, .. })
            | Self::Interop(L1BlockInfoInterop { batcher_address, .. }) => *batcher_address,
        }
    }

    /// Returns the sequence number for the info transaction
    pub const fn sequence_number(&self) -> u64 {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { sequence_number, .. })
            | Self::Ecotone(L1BlockInfoEcotone { sequence_number, .. })
            | Self::Interop(L1BlockInfoInterop { sequence_number, .. }) => *sequence_number,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;
    use alloy_primitives::{address, b256, hex};

    const RAW_BEDROCK_INFO_TX: [u8; L1BlockInfoBedrock::L1_INFO_TX_LEN] = hex!("015d8eb9000000000000000000000000000000000000000000000000000000000117c4eb0000000000000000000000000000000000000000000000000000000065280377000000000000000000000000000000000000000000000000000000026d05d953392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc00000000000000000000000000000000000000000000000000000000000000040000000000000000000000006887246668a3b87f54deb3b94ba47a6f63f3298500000000000000000000000000000000000000000000000000000000000000bc00000000000000000000000000000000000000000000000000000000000a6fe0");
    const RAW_ECOTONE_INFO_TX: [u8; L1BlockInfoEcotone::L1_INFO_TX_LEN] = hex!("440a5e2000000558000c5fc5000000000000000500000000661c277300000000012bec20000000000000000000000000000000000000000000000000000000026e9f109900000000000000000000000000000000000000000000000000000000000000011c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add30000000000000000000000006887246668a3b87f54deb3b94ba47a6f63f32985");
    const RAW_INTEROP_INFO_TX: [u8; L1BlockInfoInterop::L1_INFO_TX_LEN] = hex!("760ee04d00000558000c5fc50000000000000001000000006789ab380000000000000000000000000000000000000000000000000000000000000000000000003b9aca0000000000000000000000000000000000000000000000000000000000000000014f98b83baf52c498b49bfff33e59965b27da7febbea9a2fcc4719d06dc06932a000000000000000000000000c0658ee336b551ff83216fbdf85ec92613d23602");

    #[test]
    fn bedrock_l1_block_info_invalid_len() {
        let err = L1BlockInfoBedrock::decode_calldata(&[0xde, 0xad]);
        assert!(err.is_err());
        assert_eq!(
            err.err().unwrap().to_string(),
            "Invalid data length: Invalid calldata length for Bedrock L1 info transaction, expected 260, got 2"
        );
    }

    #[test]
    fn ecotone_l1_block_info_invalid_len() {
        let err = L1BlockInfoEcotone::decode_calldata(&[0xde, 0xad]);
        assert!(err.is_err());
        assert_eq!(
            err.err().unwrap().to_string(),
            "Invalid data length: Invalid calldata length for Ecotone L1 info transaction, expected 164, got 2"
        );
    }

    #[test]
    fn interop_l1_block_info_invalid_len() {
        let err = L1BlockInfoInterop::decode_calldata(&[0xde, 0xad]);
        assert!(err.is_err());
        assert_eq!(
            err.err().unwrap().to_string(),
            "Invalid data length: Invalid calldata length for Interop L1 info transaction, expected 164, got 2"
        );
    }

    #[test]
    fn test_l1_block_info_tx_block_hash_bedrock() {
        let bedrock = L1BlockInfoTx::Bedrock(L1BlockInfoBedrock {
            block_hash: b256!("392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc"),
            ..Default::default()
        });
        assert_eq!(
            bedrock.block_hash(),
            b256!("392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc")
        );
    }

    #[test]
    fn test_l1_block_info_tx_block_hash_ecotone() {
        let ecotone = L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            block_hash: b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3"),
            ..Default::default()
        });
        assert_eq!(
            ecotone.block_hash(),
            b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3")
        );
    }

    #[test]
    fn test_l1_block_info_tx_block_hash_interop() {
        let interop = L1BlockInfoTx::Interop(L1BlockInfoInterop {
            block_hash: b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3"),
            ..Default::default()
        });
        assert_eq!(
            interop.block_hash(),
            b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3")
        );
    }

    #[test]
    fn test_l1_base_fee() {
        let bedrock =
            L1BlockInfoTx::Bedrock(L1BlockInfoBedrock { base_fee: 123, ..Default::default() });
        assert_eq!(bedrock.l1_base_fee(), U256::from(123));

        let ecotone =
            L1BlockInfoTx::Ecotone(L1BlockInfoEcotone { base_fee: 456, ..Default::default() });
        assert_eq!(ecotone.l1_base_fee(), U256::from(456));

        let interop =
            L1BlockInfoTx::Interop(L1BlockInfoInterop { base_fee: 789, ..Default::default() });
        assert_eq!(interop.l1_base_fee(), U256::from(789));
    }

    #[test]
    fn test_l1_fee_overhead() {
        let bedrock = L1BlockInfoTx::Bedrock(L1BlockInfoBedrock {
            l1_fee_overhead: U256::from(123),
            ..Default::default()
        });
        assert_eq!(bedrock.l1_fee_overhead(), U256::from(123));

        let ecotone = L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            l1_fee_overhead: U256::from(456),
            ..Default::default()
        });
        assert_eq!(ecotone.l1_fee_overhead(), U256::from(456));

        let interop = L1BlockInfoTx::Interop(L1BlockInfoInterop::default());
        assert_eq!(interop.l1_fee_overhead(), U256::ZERO);
    }

    #[test]
    fn test_l1_fee_scalar() {
        let bedrock = L1BlockInfoTx::Bedrock(L1BlockInfoBedrock {
            l1_fee_scalar: U256::from(123),
            ..Default::default()
        });
        assert_eq!(bedrock.l1_fee_scalar(), U256::from(123));

        let ecotone = L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            base_fee_scalar: 456,
            ..Default::default()
        });
        assert_eq!(ecotone.l1_fee_scalar(), U256::from(456));

        let interop = L1BlockInfoTx::Interop(L1BlockInfoInterop {
            base_fee_scalar: 789,
            ..Default::default()
        });
        assert_eq!(interop.l1_fee_scalar(), U256::from(789));
    }

    #[test]
    fn test_blob_base_fee() {
        let bedrock = L1BlockInfoTx::Bedrock(L1BlockInfoBedrock { ..Default::default() });
        assert_eq!(bedrock.blob_base_fee(), U256::ZERO);

        let ecotone =
            L1BlockInfoTx::Ecotone(L1BlockInfoEcotone { blob_base_fee: 456, ..Default::default() });
        assert_eq!(ecotone.blob_base_fee(), U256::from(456));

        let interop =
            L1BlockInfoTx::Interop(L1BlockInfoInterop { blob_base_fee: 789, ..Default::default() });
        assert_eq!(interop.blob_base_fee(), U256::from(789));
    }

    #[test]
    fn test_blob_base_fee_scalar() {
        let bedrock = L1BlockInfoTx::Bedrock(L1BlockInfoBedrock { ..Default::default() });
        assert_eq!(bedrock.blob_base_fee_scalar(), U256::ZERO);

        let ecotone = L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            blob_base_fee_scalar: 456,
            ..Default::default()
        });
        assert_eq!(ecotone.blob_base_fee_scalar(), U256::from(456));

        let interop = L1BlockInfoTx::Interop(L1BlockInfoInterop {
            blob_base_fee_scalar: 789,
            ..Default::default()
        });
        assert_eq!(interop.blob_base_fee_scalar(), U256::from(789));
    }

    #[test]
    fn test_empty_scalars() {
        let bedrock = L1BlockInfoTx::Bedrock(L1BlockInfoBedrock { ..Default::default() });
        assert!(!bedrock.empty_scalars());

        let ecotone = L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            empty_scalars: true,
            ..Default::default()
        });
        assert!(ecotone.empty_scalars());

        let ecotone = L1BlockInfoTx::Ecotone(L1BlockInfoEcotone::default());
        assert!(!ecotone.empty_scalars());
    }

    #[test]
    fn bedrock_l1_block_info_tx_roundtrip() {
        let expected = L1BlockInfoBedrock {
            number: 18334955,
            time: 1697121143,
            base_fee: 10419034451,
            block_hash: b256!("392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc"),
            sequence_number: 4,
            batcher_address: address!("6887246668a3b87f54deb3b94ba47a6f63f32985"),
            l1_fee_overhead: U256::from(0xbc),
            l1_fee_scalar: U256::from(0xa6fe0),
        };

        let L1BlockInfoTx::Bedrock(decoded) =
            L1BlockInfoTx::decode_calldata(RAW_BEDROCK_INFO_TX.as_ref()).unwrap()
        else {
            panic!("Wrong fork");
        };
        assert_eq!(expected, decoded);
        assert_eq!(RAW_BEDROCK_INFO_TX, decoded.encode_calldata().as_ref());
    }

    #[test]
    fn ecotone_l1_block_info_tx_roundtrip() {
        let expected = L1BlockInfoEcotone {
            number: 19655712,
            time: 1713121139,
            base_fee: 10445852825,
            block_hash: b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3"),
            sequence_number: 5,
            batcher_address: address!("6887246668a3b87f54deb3b94ba47a6f63f32985"),
            blob_base_fee: 1,
            blob_base_fee_scalar: 810949,
            base_fee_scalar: 1368,
            empty_scalars: false,
            l1_fee_overhead: U256::ZERO,
        };

        let L1BlockInfoTx::Ecotone(decoded) =
            L1BlockInfoTx::decode_calldata(RAW_ECOTONE_INFO_TX.as_ref()).unwrap()
        else {
            panic!("Wrong fork");
        };
        assert_eq!(expected, decoded);
        assert_eq!(decoded.encode_calldata().as_ref(), RAW_ECOTONE_INFO_TX);
    }

    #[test]
    fn interop_l1_block_info_tx_roundtrip() {
        let expected = L1BlockInfoInterop {
            number: 0,
            time: 1737075512,
            base_fee: 1000000000,
            block_hash: b256!("4f98b83baf52c498b49bfff33e59965b27da7febbea9a2fcc4719d06dc06932a"),
            sequence_number: 1,
            batcher_address: address!("c0658ee336b551ff83216fbdf85ec92613d23602"),
            blob_base_fee: 1,
            blob_base_fee_scalar: 810949,
            base_fee_scalar: 1368,
        };

        let L1BlockInfoTx::Interop(decoded) =
            L1BlockInfoTx::decode_calldata(RAW_INTEROP_INFO_TX.as_ref()).unwrap()
        else {
            panic!("Wrong fork");
        };
        assert_eq!(expected, decoded);
        assert_eq!(decoded.encode_calldata().as_ref(), RAW_INTEROP_INFO_TX);
    }

    #[test]
    fn try_new_with_deposit_tx_bedrock() {
        let rollup_config = RollupConfig::default();
        let system_config = SystemConfig::default();
        let sequence_number = 0;
        let l1_header = Header::default();
        let l2_block_time = 0;

        let l1_info = L1BlockInfoTx::try_new(
            &rollup_config,
            &system_config,
            sequence_number,
            &l1_header,
            l2_block_time,
        )
        .unwrap();

        let L1BlockInfoTx::Bedrock(l1_info) = l1_info else {
            panic!("Wrong fork");
        };

        assert_eq!(l1_info.number, l1_header.number);
        assert_eq!(l1_info.time, l1_header.timestamp);
        assert_eq!(l1_info.base_fee, { l1_header.base_fee_per_gas.unwrap_or(0) });
        assert_eq!(l1_info.block_hash, l1_header.hash_slow());
        assert_eq!(l1_info.sequence_number, sequence_number);
        assert_eq!(l1_info.batcher_address, system_config.batcher_address);
        assert_eq!(l1_info.l1_fee_overhead, system_config.overhead);
        assert_eq!(l1_info.l1_fee_scalar, system_config.scalar);
    }

    #[test]
    fn try_new_with_deposit_tx_ecotone() {
        let rollup_config = RollupConfig { ecotone_time: Some(1), ..Default::default() };
        let system_config = SystemConfig::default();
        let sequence_number = 0;
        let l1_header = Header::default();
        let l2_block_time = 0xFF;

        let l1_info = L1BlockInfoTx::try_new(
            &rollup_config,
            &system_config,
            sequence_number,
            &l1_header,
            l2_block_time,
        )
        .unwrap();

        let L1BlockInfoTx::Ecotone(l1_info) = l1_info else {
            panic!("Wrong fork");
        };

        assert_eq!(l1_info.number, l1_header.number);
        assert_eq!(l1_info.time, l1_header.timestamp);
        assert_eq!(l1_info.base_fee, { l1_header.base_fee_per_gas.unwrap_or(0) });
        assert_eq!(l1_info.block_hash, l1_header.hash_slow());
        assert_eq!(l1_info.sequence_number, sequence_number);
        assert_eq!(l1_info.batcher_address, system_config.batcher_address);
        assert_eq!(l1_info.blob_base_fee, l1_header.blob_fee(BlobParams::cancun()).unwrap_or(1));

        let scalar = system_config.scalar.to_be_bytes::<32>();
        let blob_base_fee_scalar = (scalar[0] == L1BlockInfoEcotone::L1_SCALAR)
            .then(|| {
                u32::from_be_bytes(
                    scalar[24..28].try_into().expect("Failed to parse L1 blob base fee scalar"),
                )
            })
            .unwrap_or_default();
        let base_fee_scalar =
            u32::from_be_bytes(scalar[28..32].try_into().expect("Failed to parse base fee scalar"));
        assert_eq!(l1_info.blob_base_fee_scalar, blob_base_fee_scalar);
        assert_eq!(l1_info.base_fee_scalar, base_fee_scalar);
    }

    #[test]
    fn try_new_with_deposit_tx_interop() {
        let rollup_config = RollupConfig { interop_time: Some(1), ..Default::default() };
        let system_config = SystemConfig::default();
        let sequence_number = 0;
        let l1_header = Header::default();
        let l2_block_time = 0xFF;

        let l1_info = L1BlockInfoTx::try_new(
            &rollup_config,
            &system_config,
            sequence_number,
            &l1_header,
            l2_block_time,
        )
        .unwrap();

        let L1BlockInfoTx::Interop(l1_info) = l1_info else {
            panic!("Wrong fork");
        };

        assert_eq!(l1_info.number, l1_header.number);
        assert_eq!(l1_info.time, l1_header.timestamp);
        assert_eq!(l1_info.base_fee, { l1_header.base_fee_per_gas.unwrap_or(0) });
        assert_eq!(l1_info.block_hash, l1_header.hash_slow());
        assert_eq!(l1_info.sequence_number, sequence_number);
        assert_eq!(l1_info.batcher_address, system_config.batcher_address);
        assert_eq!(l1_info.blob_base_fee, l1_header.blob_fee(BlobParams::cancun()).unwrap_or(1));

        let scalar = system_config.scalar.to_be_bytes::<32>();
        let blob_base_fee_scalar = (scalar[0] == L1BlockInfoInterop::L1_SCALAR)
            .then(|| {
                u32::from_be_bytes(
                    scalar[24..28].try_into().expect("Failed to parse L1 blob base fee scalar"),
                )
            })
            .unwrap_or_default();
        let base_fee_scalar =
            u32::from_be_bytes(scalar[28..32].try_into().expect("Failed to parse base fee scalar"));
        assert_eq!(l1_info.blob_base_fee_scalar, blob_base_fee_scalar);
        assert_eq!(l1_info.base_fee_scalar, base_fee_scalar);
    }
}
