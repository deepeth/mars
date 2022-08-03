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

use std::sync::Arc;

use arrow2::array::Array;
use arrow2::array::ListArray;
use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::buffer::Buffer;
use arrow2::chunk::Chunk;
use arrow2::datatypes::DataType;
use arrow2::datatypes::Field;
use arrow2::datatypes::Schema;
use common_exceptions::Result;
use web3::types::TransactionReceipt;

use crate::contexts::ContextRef;
use crate::exporters::bytes_to_hex;
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
        let mut data_vec = Vec::new();
        let mut topics_vec = Vec::new();
        let mut topic_offsets_vec = Vec::new();
        let mut offset = 0i32;
        topic_offsets_vec.push(offset);

        for (idx, receipt) in receipts.iter().enumerate() {
            for log in &receipt.logs {
                log_index_vec.push(idx as u64);
                transaction_hash_vec.push(format!("{:#x}", receipt.transaction_hash));
                transaction_index_vec.push(receipt.transaction_index.as_u64());
                data_vec.push(format!("0x{:}", &bytes_to_hex(&log.data)));

                for topic in &log.topics {
                    topics_vec.push(format!("{:#x}", topic));
                }
                offset += log.topics.len() as i32;
                topic_offsets_vec.push(offset);
            }
        }
        let log_index_array = UInt64Array::from_slice(log_index_vec);
        let transaction_hash_array = Utf8Array::<i32>::from_slice(transaction_hash_vec);
        let transaction_index_array = UInt64Array::from_slice(transaction_index_vec);
        let data_array = Utf8Array::<i32>::from_slice(data_vec);

        // log topics.
        let topics_values = Utf8Array::<i32>::from_slice(topics_vec);
        let topics_array = ListArray::<i32>::from_data(
            DataType::Utf8,
            Buffer::from(topic_offsets_vec),
            Arc::new(topics_values),
            None,
        );

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
        let data_field = Field::new("data", data_array.data_type().clone(), true);
        let topics_field = Field::new("topics", topics_array.data_type().clone(), true);

        let schema = Schema::from(vec![
            log_index_field,
            transaction_hash_field,
            transaction_index_field,
            data_field,
            topics_field,
        ]);
        let columns = Chunk::try_new(vec![
            log_index_array.boxed(),
            transaction_hash_array.boxed(),
            transaction_index_array.boxed(),
            data_array.boxed(),
            topics_array.boxed(),
        ])?;

        let path = format!("{}/logs", self.dir);
        write_file(&self.ctx, &path, schema, columns, "logs").await
    }
}
