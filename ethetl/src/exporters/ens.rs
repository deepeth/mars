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

use arrow2::array::Int128Array;
use arrow2::array::Int64Array;
use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::DataType;
use arrow2::datatypes::Field;
use arrow2::datatypes::Schema;
use arrow2::datatypes::TimeUnit::Second;
use common_eth::decode_name_registered_data;
use common_eth::h256_to_hex;
use common_eth::ENS_NAME_REGISTERED_SIG;
use common_exceptions::Result;
use web3::types::Log;
use web3::types::TransactionReceipt;
use web3::types::H256;
use web3::types::U256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::exporters::write_file;

struct Ens {
    name: String,
    cost: U256,
    expires: u64,
    owner: String,
}

pub struct EnsExporter {
    ctx: ContextRef,
    output_dir: String,
    range_path: String,
    receipts: Vec<TransactionReceipt>,
}

impl EnsExporter {
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

    fn parse_log(log: &Log) -> Result<Option<Ens>> {
        let topics = &log.topics;
        if topics.len() < 2 {
            return Ok(None);
        }

        let topic_0 = h256_to_hex(&topics[0]);
        if ENS_NAME_REGISTERED_SIG == topic_0.as_str() {
            if let Some((name, cost, expires)) = decode_name_registered_data(&log.data)? {
                let owner = h256_to_hex(&topics[2]);
                return Ok(Some(Ens {
                    name,
                    cost,
                    expires: expires.as_u64(),
                    owner,
                }));
            }
        }

        Ok(None)
    }

    pub async fn export(&self) -> Result<()> {
        let mut name_vec = vec![];
        let mut cost_vec = vec![];
        let mut expires_vec = vec![];
        let mut owner_vec = vec![];
        let mut transaction_hash_vec = vec![];
        let mut block_number_vec = vec![];

        for receipt in &self.receipts {
            for logs in &receipt.logs {
                if let Some(ens) = Self::parse_log(logs)? {
                    name_vec.push(ens.name);
                    cost_vec.push(ens.cost.as_u128() as i128);
                    expires_vec.push(ens.expires as i64);
                    owner_vec.push(ens.owner);
                    transaction_hash_vec.push(h256_to_hex(
                        &logs.transaction_hash.unwrap_or_else(H256::zero),
                    ));
                    block_number_vec.push(logs.block_number.unwrap_or_else(U64::zero).as_u64());

                    self.ctx.get_progress().incr_ens(1);
                }
            }
        }

        let name_array = Utf8Array::<i32>::from_slice(name_vec);
        let cost_array = Int128Array::from_slice(cost_vec).to(DataType::Decimal(36, 18));
        let expires_array =
            Int64Array::from_slice(expires_vec).to(DataType::Timestamp(Second, None));
        let owner_array = Utf8Array::<i32>::from_slice(owner_vec);
        let transaction_hash_array = Utf8Array::<i32>::from_slice(transaction_hash_vec);
        let block_number_array = UInt64Array::from_slice(block_number_vec);

        let name_field = Field::new("name", name_array.data_type().clone(), true);
        let cost_field = Field::new("cost", cost_array.data_type().clone(), true);
        let expires_field = Field::new("expires", expires_array.data_type().clone(), true);
        let owner_field = Field::new("owner", owner_array.data_type().clone(), true);
        let transaction_hash_field = Field::new(
            "transaction_hash",
            transaction_hash_array.data_type().clone(),
            true,
        );
        let block_number_field =
            Field::new("block_number", block_number_array.data_type().clone(), true);
        let schema = Schema::from(vec![
            name_field,
            cost_field,
            expires_field,
            owner_field,
            transaction_hash_field,
            block_number_field,
        ]);
        let columns = Chunk::try_new(vec![
            name_array.boxed(),
            cost_array.boxed(),
            expires_array.boxed(),
            owner_array.boxed(),
            transaction_hash_array.boxed(),
            block_number_array.boxed(),
        ])?;

        let path = format!("{}/ens/ens_{}", self.output_dir, self.range_path);
        write_file(&self.ctx, &path, schema, columns, "ens").await
    }
}
