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
use common_exceptions::Result;
use web3::types::TransactionReceipt;
use web3::types::H256;
use web3::types::U256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::exporters::h256_to_hex;
use crate::exporters::write_file;
use crate::exporters::TOKEN_TRANSFER_CONTRACT_ADDRESS_HEX;

pub struct NftTransferExporter {
    ctx: ContextRef,
    dir: String,
    receipts: Vec<TransactionReceipt>,
}

impl NftTransferExporter {
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
        let mut token_id_vec = vec![];
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

                // NFT token transfer contract address.
                let topic_0 = format!("{:#x}", topics[0]);
                if topic_0.as_str() == TOKEN_TRANSFER_CONTRACT_ADDRESS_HEX && topics.len() == 4 {
                    token_address_vec.push(format!("{:#x}", logs.address));
                    from_address_vec.push(format!("0x{}", h256_to_hex(&topics[1])));
                    to_address_vec.push(format!("0x{}", h256_to_hex(&topics[2])));
                    token_id_vec.push(format!("0x{}", h256_to_hex(&topics[3])));
                    erc_standard_vec.push("ERC20");
                    transaction_hash_vec.push(format!(
                        "{:#x}",
                        logs.transaction_hash.unwrap_or_else(H256::zero)
                    ));
                    log_index_vec.push(logs.log_index.unwrap_or_else(U256::zero).as_u64());
                    block_number_vec.push(logs.block_number.unwrap_or_else(U64::zero).as_u64());
                }
            }
        }

        let token_address_array = Utf8Array::<i32>::from_slice(token_address_vec);
        let from_address_array = Utf8Array::<i32>::from_slice(from_address_vec);
        let to_address_array = Utf8Array::<i32>::from_slice(to_address_vec);
        let token_id_array = Utf8Array::<i32>::from_slice(token_id_vec);
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
            erc_standard_array.boxed(),
            transaction_hash_array.boxed(),
            log_index_array.boxed(),
            block_number_array.boxed(),
        ])?;

        let receipt_path = format!("{}/nft_token_transfers", self.dir);
        write_file(
            &self.ctx,
            &receipt_path,
            schema,
            columns,
            "nft_token_transfers",
        )
        .await
    }
}
