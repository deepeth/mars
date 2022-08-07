use ethereum_types::Address;
use ethereum_types::H256;
use ethereum_types::U256;
use ethereum_types::U64;
use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::types::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListEntry {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum TransactionMessage {
    #[serde(rename = "0x0")]
    #[serde(rename_all = "camelCase")]
    Legacy {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        chain_id: Option<U64>,
        nonce: U64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        gas: U64,
        gas_price: U256,
        value: U256,
        input: Bytes,
    },
    #[serde(rename = "0x1")]
    #[serde(rename_all = "camelCase")]
    EIP2930 {
        chain_id: U64,
        nonce: U64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        gas: U64,
        gas_price: U256,
        value: U256,
        input: Bytes,
        access_list: Vec<AccessListEntry>,
    },
    #[serde(rename = "0x2")]
    #[serde(rename_all = "camelCase")]
    EIP1559 {
        chain_id: U64,
        nonce: U64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        gas: U64,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: U256,
        value: U256,
        input: Bytes,
        access_list: Vec<AccessListEntry>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(flatten)]
    pub message: TransactionMessage,
    /// RLP encoded representation of the transaction.
    pub v: U64,
    pub r: H256,
    pub s: H256,

    pub from: Address,
    pub hash: H256,
    pub transaction_index: Option<U64>,
    pub block_number: Option<U64>,
    pub block_hash: Option<H256>,
}
