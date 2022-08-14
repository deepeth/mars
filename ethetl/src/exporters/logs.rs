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
use common_exceptions::Result;
use web3::types::Address;
use web3::types::TransactionReceipt;
use web3::types::H256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::exporters::write_file;

pub struct LogsExporter {
    ctx: ContextRef,
    dir: String,
    receipts: Vec<TransactionReceipt>,
}

impl LogsExporter {
    pub fn create(ctx: &ContextRef, dir: &str, receipts: &[TransactionReceipt]) -> LogsExporter {
        Self {
            ctx: ctx.clone(),
            dir: dir.to_string(),
            receipts: receipts.to_vec(),
        }
    }

    pub async fn export(&self) -> Result<()> {
        let receipts = &self.receipts;

        let mut log_index_vec = Vec::new();
        let mut transaction_hash_vec = Vec::new();
        let mut transaction_index_vec = Vec::new();
        let mut block_hash_vec = Vec::new();
        let mut block_number_vec = Vec::new();
        let mut contract_address_vec = Vec::new();
        let mut data_vec = Vec::new();
        let mut topics_vec = Vec::new();

        for (idx, receipt) in receipts.iter().enumerate() {
            for log in &receipt.logs {
                log_index_vec.push(idx as u64);
                transaction_hash_vec.push(h256_to_hex(&receipt.transaction_hash));
                transaction_index_vec.push(receipt.transaction_index.as_u64());
                block_hash_vec.push(h256_to_hex(&receipt.block_hash.unwrap_or_else(H256::zero)));
                block_number_vec.push(receipt.block_number.unwrap_or_else(U64::zero).as_u64());
                contract_address_vec.push(h160_to_hex(
                    &receipt.contract_address.unwrap_or_else(Address::zero),
                ));
                data_vec.push(bytes_to_hex(&log.data));
                let topics = log
                    .topics
                    .iter()
                    .map(h256_to_hex)
                    .collect::<Vec<String>>()
                    .join("|");
                topics_vec.push(topics);

                self.ctx.get_progress().incr_logs(1);
            }
        }
        let log_index_array = UInt64Array::from_slice(log_index_vec);
        let transaction_hash_array = Utf8Array::<i32>::from_slice(transaction_hash_vec);
        let transaction_index_array = UInt64Array::from_slice(transaction_index_vec);
        let block_hash_array = Utf8Array::<i32>::from_slice(block_hash_vec);
        let block_number_array = UInt64Array::from_slice(block_number_vec);
        let contract_address_array = Utf8Array::<i32>::from_slice(contract_address_vec);
        let data_array = Utf8Array::<i32>::from_slice(data_vec);
        let topics_array = Utf8Array::<i32>::from_slice(topics_vec);

        let log_index_field = Field::new("log_index", log_index_array.data_type().clone(), true);
        let transaction_hash_field = Field::new(
            "transaction_hash",
            transaction_hash_array.data_type().clone(),
            true,
        );
        let transaction_index_field = Field::new(
            "transaction_index",
            transaction_index_array.data_type().clone(),
            true,
        );
        let block_hash_field = Field::new("block_hash", block_hash_array.data_type().clone(), true);
        let block_number_field =
            Field::new("block_number", block_number_array.data_type().clone(), true);
        let contracet_address_field = Field::new(
            "contract_address",
            contract_address_array.data_type().clone(),
            true,
        );
        let data_field = Field::new("data", data_array.data_type().clone(), true);
        let topics_field = Field::new("topics", topics_array.data_type().clone(), true);

        let schema = Schema::from(vec![
            log_index_field,
            transaction_hash_field,
            transaction_index_field,
            block_hash_field,
            block_number_field,
            contracet_address_field,
            data_field,
            topics_field,
        ]);
        let columns = Chunk::try_new(vec![
            log_index_array.boxed(),
            transaction_hash_array.boxed(),
            transaction_index_array.boxed(),
            block_hash_array.boxed(),
            block_number_array.boxed(),
            contract_address_array.boxed(),
            data_array.boxed(),
            topics_array.boxed(),
        ])?;

        let path = format!("{}/logs", self.dir);
        write_file(&self.ctx, &path, schema, columns, "logs").await
    }
}
