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
use common_eth::bytes_to_hex;
use common_eth::h160_to_hex;
use common_eth::h256_to_hex;
use common_eth::ERC20_TOKEN_TRANSFER_SIG;
use common_exceptions::Result;
use web3::types::TransactionReceipt;
use web3::types::H256;
use web3::types::U256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::exporters::write_file;

pub struct TokenTransferExporter {
    ctx: ContextRef,
    dir: String,
    receipts: Vec<TransactionReceipt>,
}

impl TokenTransferExporter {
    pub fn create(ctx: &ContextRef, dir: &str, receipts: &[TransactionReceipt]) -> Self {
        Self {
            ctx: ctx.clone(),
            dir: dir.to_string(),
            receipts: receipts.to_vec(),
        }
    }

    pub async fn export(&self) -> Result<()> {
        let mut token_address_vec = vec![];
        let mut from_address_vec = vec![];
        let mut to_address_vec = vec![];
        let mut data_vec = vec![];
        let mut erc_standard_vec = vec![];
        let mut transaction_hash_vec = vec![];
        let mut log_index_vec = vec![];
        let mut block_number_vec = vec![];

        for receipt in &self.receipts {
            for logs in &receipt.logs {
                let topics = &logs.topics;
                if topics.is_empty() {
                    continue;
                }

                // Token transfer contract address.
                let topic_0 = format!("{:#x}", topics[0]);
                if topic_0.as_str() == ERC20_TOKEN_TRANSFER_SIG {
                    if topics.len() == 3 {
                        from_address_vec.push(h256_to_hex(&topics[1]));
                        to_address_vec.push(h256_to_hex(&topics[2]));
                        data_vec.push(bytes_to_hex(&logs.data));
                        erc_standard_vec.push("ERC20");
                    } else if topics.len() == 4 {
                        from_address_vec.push(h256_to_hex(&topics[1]));
                        to_address_vec.push(h256_to_hex(&topics[2]));
                        data_vec.push(h256_to_hex(&topics[3]));
                        erc_standard_vec.push("ERC721");
                    } else {
                        from_address_vec.push("".to_string());
                        to_address_vec.push("".to_string());
                        data_vec.push("".to_string());
                        erc_standard_vec.push("");
                    }
                    token_address_vec.push(h160_to_hex(&logs.address));
                    transaction_hash_vec.push(h256_to_hex(
                        &logs.transaction_hash.unwrap_or_else(H256::zero),
                    ));
                    log_index_vec.push(logs.log_index.unwrap_or_else(U256::zero).as_u64());
                    block_number_vec.push(logs.block_number.unwrap_or_else(U64::zero).as_u64());

                    self.ctx.get_progress().incr_token_transfers(1);
                }
                // TODO(ERC1155)
            }
        }

        let token_address_array = Utf8Array::<i32>::from_slice(token_address_vec);
        let from_address_array = Utf8Array::<i32>::from_slice(from_address_vec);
        let to_address_array = Utf8Array::<i32>::from_slice(to_address_vec);
        let data_array = Utf8Array::<i32>::from_slice(data_vec);
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
        let data_field = Field::new("data", data_array.data_type().clone(), true);
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
            data_field,
            erc_standard_field,
            transaction_hash_field,
            log_index_field,
            block_number_field,
        ]);
        let columns = Chunk::try_new(vec![
            token_address_array.boxed(),
            from_address_array.boxed(),
            to_address_array.boxed(),
            data_array.boxed(),
            erc_standard_array.boxed(),
            transaction_hash_array.boxed(),
            log_index_array.boxed(),
            block_number_array.boxed(),
        ])?;

        let path = format!("{}/token_transfers", self.dir);
        write_file(&self.ctx, &path, schema, columns, "token_transfer").await
    }
}
