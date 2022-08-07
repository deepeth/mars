use std::str::FromStr;

use ethereum_types::Address;
use ethereum_types::Bloom;
use ethereum_types::H256;
use ethereum_types::H64;
use ethereum_types::U256;
use ethereum_types::U64;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use crate::jsonrpc::types::Bytes;
use crate::jsonrpc::types::Tx;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A 64-bit unsigned integer (or tag - "latest", "earliest", "pending").
pub enum BlockNumber {
    /// Latest block.
    Latest,
    /// Earliest block (genesis).
    Earliest,
    /// Pending block (not yet part of the canonical chain).
    Pending,
    /// A block number.
    Number(U64),
}

impl Serialize for BlockNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match *self {
            BlockNumber::Number(ref n) => serializer.serialize_str(&format!("0x{:x}", n)),
            BlockNumber::Latest => serializer.serialize_str("latest"),
            BlockNumber::Earliest => serializer.serialize_str("earliest"),
            BlockNumber::Pending => serializer.serialize_str("pending"),
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?.to_lowercase();
        Ok(match s.as_str() {
            "latest" => Self::Latest,
            "earliest" => Self::Earliest,
            "pending" => Self::Pending,
            n => BlockNumber::Number(U64::from_str(n).map_err(serde::de::Error::custom)?),
        })
    }
}

impl<T: Into<U64>> From<T> for BlockNumber {
    fn from(n: T) -> Self {
        BlockNumber::Number(n.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// BlockId is either a Block Number or a Hash.
#[serde(untagged)]
pub enum BlockId {
    /// A 256-bit Hash.
    Hash(H256),
    /// A block number.
    Number(BlockNumber),
}

impl From<BlockNumber> for BlockId {
    fn from(n: BlockNumber) -> Self {
        BlockId::Number(n)
    }
}

impl From<H256> for BlockId {
    fn from(hash: H256) -> Self {
        BlockId::Hash(hash)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    /// Number of the block.
    pub number: Option<U64>,
    /// Block's hash.
    pub hash: Option<H256>,
    /// Block's parent's hash.
    pub parent_hash: H256,
    /// Hash of the block's uncles.
    pub sha3_uncles: H256,
    /// Logs bloom.
    pub logs_bloom: Option<Bloom>,
    /// Transactions root hash.
    pub transactions_root: H256,
    /// State root hash.
    pub state_root: H256,
    /// Receipts root hash.
    pub receipts_root: H256,
    /// Block's beneficiary.
    pub miner: Address,
    /// Block's PoW difficulty.
    pub difficulty: U256,
    /// Total chain's difficulty at moment of the block inclusion, none if pending.
    pub total_difficulty: Option<U256>,
    /// Seal fields.
    pub seal_fields: Option<(H256, H64)>,
    /// Block's nonce.
    pub nonce: Option<H64>,
    /// Mix hash.
    pub mix_hash: Option<H256>,
    /// Block's extra data.
    pub extra_data: Bytes,
    /// Block's size.
    pub size: U64,
    /// Block's gas limit.
    pub gas_limit: U64,
    /// Used gas of all transactions within the block.
    pub gas_used: U64,
    /// Block's timestamp.
    pub timestamp: U64,
    /// Block's transactions.
    pub transactions: Vec<Tx>,
    /// Block's uncles.
    pub uncles: Vec<H256>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_fee_per_gas: Option<U256>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub parent_hash: H256,
    pub sha3_uncles: H256,
    pub miner: Address,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: U256,
    pub gas_limit: U64,
    pub gas_used: U64,
    pub timestamp: U64,
    pub extra_data: Bytes,
    pub mix_hash: H256,
    pub nonce: H64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_fee_per_gas: Option<U256>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ser_de_block_number() {
        let block_number = BlockNumber::Earliest;
        let hexstring = r#""earliest""#;
        assert_eq!(serde_json::to_string(&block_number).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_str::<BlockNumber>(hexstring).unwrap(),
            block_number
        );

        let block_number = BlockNumber::Latest;
        let hexstring = r#""latest""#;
        assert_eq!(serde_json::to_string(&block_number).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_str::<BlockNumber>(hexstring).unwrap(),
            block_number
        );

        let block_number = BlockNumber::Pending;
        let hexstring = r#""pending""#;
        assert_eq!(serde_json::to_string(&block_number).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_str::<BlockNumber>(hexstring).unwrap(),
            block_number
        );

        let block_number = BlockNumber::Number(1.into());
        let hexstring = r#""0x1""#;
        assert_eq!(serde_json::to_string(&block_number).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_str::<BlockNumber>(hexstring).unwrap(),
            block_number
        );

        let block_number = BlockNumber::Number(0x7b.into());
        let hexstring = r#""0x7b""#;
        assert_eq!(serde_json::to_string(&block_number).unwrap(), hexstring);
    }

    #[test]
    fn test_ser_de_block_id() {
        let block_id = r#""0x7b""#;
        assert_eq!(
            serde_json::to_string(&BlockId::Number(123.into())).unwrap(),
            block_id
        );

        assert_eq!(
            serde_json::from_str::<BlockId>(block_id).unwrap(),
            BlockId::Number(123.into())
        );

        let block_hash = r#""0x0000000000000000000000000000000000000000000000000000000000000000""#;
        assert_eq!(
            serde_json::to_string(&BlockId::Hash(H256::from([0; 32]))).unwrap(),
            block_hash
        );

        assert_eq!(
            serde_json::from_str::<BlockId>(block_hash).unwrap(),
            BlockId::Hash(H256::from([0; 32]))
        );
    }
}
