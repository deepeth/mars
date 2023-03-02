// Copyright 2022 BohuTANG.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Field;
use arrow2::datatypes::Schema;
use common_eth::decode_transfer_batch_data;
use common_eth::decode_transfer_single_data;
use common_eth::decode_u256_data;
use common_eth::h160_to_hex;
use common_eth::h256_to_hex;
use common_eth::u256_to_hex;
use common_eth::ERC1155_TRANSFER_BATCH_SIG;
use common_eth::ERC1155_TRANSFER_SINGLE_SIG;
use common_eth::ERC20_TOKEN_TRANSFER_SIG;
use common_exceptions::Result;
use web3::types::Log;
use web3::types::TransactionReceipt;
use web3::types::H256;
use web3::types::U256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::exporters::eth::write_file;

struct Transfer {
    from: String,
    to: String,
    token_id: String,
    value: U256,
    erc: String,
}

pub struct TokenTransferExporter {
    ctx: ContextRef,
    output_dir: String,
    range_path: String,
    receipts: Vec<TransactionReceipt>,
}

impl TokenTransferExporter {
    pub fn create(
        ctx: &ContextRef,
        dir: &str,
        range_path: &str,
        receipts: &[TransactionReceipt],
    ) -> Self {
        Self {
            ctx: ctx.clone(),
            output_dir: dir.to_string(),
            range_path: range_path.to_string(),
            receipts: receipts.to_vec(),
        }
    }

    fn parse_log(log: &Log) -> Result<Option<Vec<Transfer>>> {
        let topics = &log.topics;
        if topics.is_empty() {
            return Ok(None);
        }

        let topic_0 = h256_to_hex(&topics[0]);
        return if ERC20_TOKEN_TRANSFER_SIG == topic_0.as_str() {
            if topics.len() == 3 {
                // Transfer (index_topic_1 address from, index_topic_2 address to, uint256 value)
                // Transfer (index_topic_1 address src, index_topic_2 address dst, uint256 wad)
                let transfer = Transfer {
                    from: h256_to_hex(&topics[1]),
                    to: h256_to_hex(&topics[2]),
                    token_id: "".to_string(),
                    value: decode_u256_data(&log.data).unwrap(),
                    erc: "ERC20".to_string(),
                };
                Ok(Some(vec![transfer]))
            } else if topics.len() == 4 {
                // Transfer (index_topic_1 address from, index_topic_2 address to, index_topic_3 uint256 tokenId)
                let transfer = Transfer {
                    from: h256_to_hex(&topics[1]),
                    to: h256_to_hex(&topics[2]),
                    token_id: h256_to_hex(&topics[3]),
                    value: U256::zero(),
                    erc: "ERC721".to_string(),
                };
                Ok(Some(vec![transfer]))
            } else {
                Ok(None)
            }
        } else if ERC1155_TRANSFER_SINGLE_SIG == topic_0.as_str() {
            // TransferSingle (index_topic_1 address operator, index_topic_2 address from, index_topic_3 address to, uint256 id, uint256 value)
            let mut u1 = U256::zero();
            let mut u2 = U256::zero();
            if let Some((x1, x2)) = decode_transfer_single_data(&log.data)? {
                u1 = x1;
                u2 = x2;
            }
            let transfer = Transfer {
                from: h256_to_hex(&topics[1]),
                to: h256_to_hex(&topics[2]),
                token_id: u256_to_hex(&u1),
                value: u2,
                erc: "ERC1155".to_string(),
            };
            Ok(Some(vec![transfer]))
        } else if ERC1155_TRANSFER_BATCH_SIG == topic_0.as_str() {
            // TransferBatch (index_topic_1 address operator, index_topic_2 address from, index_topic_3 address to, uint256[] ids, uint256[] values)
            let mut u1 = vec![];
            let mut u2 = vec![];
            if let Some((x1, x2)) = decode_transfer_batch_data(&log.data)? {
                u1 = x1;
                u2 = x2;
            }
            let mut results = vec![];
            for i in 0..u1.len() {
                let transfer = Transfer {
                    from: h256_to_hex(&topics[1]),
                    to: h256_to_hex(&topics[2]),
                    token_id: u256_to_hex(&u1[i]),
                    value: u2[i],
                    erc: "ERC1155".to_string(),
                };
                results.push(transfer);
            }
            Ok(Some(results))
        } else {
            Ok(None)
        };
    }

    pub async fn export(&self) -> Result<()> {
        let mut token_address_vec = vec![];
        let mut from_address_vec = vec![];
        let mut to_address_vec = vec![];
        let mut token_id_vec = vec![];
        let mut value_vec = vec![];
        let mut erc_standard_vec = vec![];
        let mut transaction_hash_vec = vec![];
        let mut log_index_vec = vec![];
        let mut block_number_vec = vec![];

        for receipt in &self.receipts {
            for logs in &receipt.logs {
                if let Some(transfers) = Self::parse_log(logs)? {
                    for transfer in transfers {
                        from_address_vec.push(transfer.from);
                        to_address_vec.push(transfer.to);
                        token_id_vec.push(transfer.token_id);
                        value_vec.push(transfer.value.to_string());
                        erc_standard_vec.push(transfer.erc);
                        token_address_vec.push(h160_to_hex(&logs.address));
                        transaction_hash_vec.push(h256_to_hex(
                            &logs.transaction_hash.unwrap_or_else(H256::zero),
                        ));
                        log_index_vec.push(logs.log_index.unwrap_or_else(U256::zero).as_u64());
                        block_number_vec.push(logs.block_number.unwrap_or_else(U64::zero).as_u64());

                        self.ctx.get_progress().incr_token_transfers(1);
                    }
                }
            }
        }

        let token_address_array = Utf8Array::<i32>::from_slice(token_address_vec);
        let from_address_array = Utf8Array::<i32>::from_slice(from_address_vec);
        let to_address_array = Utf8Array::<i32>::from_slice(to_address_vec);
        let token_id_array = Utf8Array::<i32>::from_slice(token_id_vec);
        let value_array = Utf8Array::<i32>::from_slice(value_vec);
        let erc_standard_array = Utf8Array::<i32>::from_slice(erc_standard_vec);
        let transaction_hash_array = Utf8Array::<i32>::from_slice(transaction_hash_vec);
        let log_index_array = UInt64Array::from_slice(log_index_vec);
        let block_number_array = UInt64Array::from_slice(block_number_vec);

        let token_address_field = Field::new(
            "token_address",
            token_address_array.data_type().clone(),
            true,
        );
        let from_address_field =
            Field::new("from_address", from_address_array.data_type().clone(), true);
        let to_address_field = Field::new("to_address", to_address_array.data_type().clone(), true);
        let token_id_field = Field::new("token_id", token_id_array.data_type().clone(), true);
        let value_field = Field::new("value", value_array.data_type().clone(), true);
        let erc_standard_field =
            Field::new("erc_standard", erc_standard_array.data_type().clone(), true);
        let transaction_hash_field = Field::new(
            "transaction_hash",
            transaction_hash_array.data_type().clone(),
            true,
        );
        let log_index_field = Field::new("log_index", log_index_array.data_type().clone(), true);
        let block_number_field =
            Field::new("block_number", block_number_array.data_type().clone(), true);
        let schema = Schema::from(vec![
            token_address_field,
            from_address_field,
            to_address_field,
            token_id_field,
            value_field,
            erc_standard_field,
            transaction_hash_field,
            log_index_field,
            block_number_field,
        ]);
        let columns = Chunk::try_new(vec![
            token_address_array.boxed(),
            from_address_array.boxed(),
            to_address_array.boxed(),
            token_id_array.boxed(),
            value_array.boxed(),
            erc_standard_array.boxed(),
            transaction_hash_array.boxed(),
            log_index_array.boxed(),
            block_number_array.boxed(),
        ])?;

        let path = format!(
            "{}/token_transfers/token_transfers_{}",
            self.output_dir, self.range_path
        );
        write_file(&self.ctx, &path, schema, columns, "token_transfer").await
    }
}
