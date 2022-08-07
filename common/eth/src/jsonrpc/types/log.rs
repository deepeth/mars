use ethereum_types::Address;
use ethereum_types::H256;
use ethereum_types::U64;
use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::types::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Transaction's log entry.
pub struct TransactionLog {
    /// Log's index within transaction.
    pub log_index: Option<U64>,
    /// Transaction's index within block.
    pub transaction_index: Option<U64>,
    /// Transaction's hash.
    pub transaction_hash: Option<H256>,
    /// Block's hash, transaction is included in.
    pub block_hash: Option<H256>,
    /// Block number, transaction is included in.
    pub block_number: Option<U64>,
    /// Log's address.
    pub address: Address,
    /// Log's data.
    pub data: Bytes,
    /// Log's Topics.
    pub topics: Vec<H256>,
}
