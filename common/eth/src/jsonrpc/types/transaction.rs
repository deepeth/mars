use ethereum_types::Address;
use ethereum_types::H256;
use ethereum_types::U256;
use ethereum_types::U64;
use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListEntry {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum MessageCall {
    #[serde(rename_all = "camelCase")]
    Legacy {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas: Option<U64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas_price: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<Bytes>,
    },
    #[serde(rename_all = "camelCase")]
    EIP2930 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas: Option<U64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas_price: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<Bytes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        access_list: Option<Vec<AccessListEntry>>,
    },
    #[serde(rename_all = "camelCase")]
    EIP1559 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas: Option<U64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_fee_per_gas: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_priority_fee_per_gas: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<Bytes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        access_list: Option<Vec<AccessListEntry>>,
    },
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// Tx is either a transaction or a transaction hash.
pub enum Tx {
    /// Transaction.
    Transaction(Box<Transaction>),
    /// Transaction hash.
    Hash(H256),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_ser_de_hexbytes_option() {
        let call_data = MessageCall::Legacy {
            from: None,
            to: Some(Address::from([0; 20])),
            gas: None,
            gas_price: None,
            value: None,
            data: None,
        };
        let hexstring = json!({
            "to":"0x0000000000000000000000000000000000000000",
        });
        assert_eq!(serde_json::to_value(&call_data).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_value::<MessageCall>(hexstring).unwrap(),
            call_data
        );

        let call_data_with_data = MessageCall::Legacy {
            from: None,
            to: Some(Address::from([0; 20])),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes::from(&b"Hello Akula"[..])),
        };

        let hexstring_with_data = json!({
            "to":"0x0000000000000000000000000000000000000000",
            "data":"0x48656c6c6f20416b756c61",
        });
        assert_eq!(
            serde_json::to_value(&call_data_with_data).unwrap(),
            hexstring_with_data
        );
        assert_eq!(
            serde_json::from_value::<MessageCall>(hexstring_with_data).unwrap(),
            call_data_with_data
        );
    }
}
