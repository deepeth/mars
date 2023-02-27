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

use std::io::Cursor;
use std::io::Write;

use arrow2::array::Int128Array;
use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::DataType;
use arrow2::datatypes::Field;
use arrow2::datatypes::Schema;
use common_eth::bytes_to_hex;
use common_eth::h160_to_hex;
use common_eth::h256_to_hex;
use common_eth::u256_to_hex;
use common_exceptions::Result;
use web3::ethabi::Address;
use web3::types::Block;
use web3::types::Transaction;
use web3::types::H256;
use web3::types::U256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::exporters::write_file;

pub struct TransactionExporter {
    ctx: ContextRef,
    output_dir: String,
    range_path: String,
    blocks: Vec<Block<Transaction>>,
}

impl TransactionExporter {
    pub fn create(
        ctx: &ContextRef,
        dir: &str,
        range_path: &str,
        blocks: &[Block<Transaction>],
    ) -> Self {
        Self {
            ctx: ctx.clone(),
            output_dir: dir.to_string(),
            range_path: range_path.to_string(),
            blocks: blocks.to_vec(),
        }
    }

    pub async fn export(&self) -> Result<()> {
        let blocks = &self.blocks;

        let mut hash_vec = vec![];
        let mut nonce_vec = vec![];
        let mut transaction_index_vec = vec![];
        let mut from_address_vec = vec![];
        let mut to_address_vec = vec![];
        let mut value_vec = vec![];
        let mut gas_vec = vec![];
        let mut gas_price_vec = vec![];
        let mut method_id_vec = vec![];
        let mut input_vec = vec![];
        let mut max_fee_per_gas_vec = vec![];
        let mut max_priority_fee_per_gas_vec = vec![];
        let mut transaction_type_vec = vec![];
        let mut block_hash_vec = vec![];
        let mut block_number_vec = vec![];
        let mut block_timestamp_vec = vec![];

        for block in blocks {
            for tx in &block.transactions {
                hash_vec.push(h256_to_hex(&tx.hash));
                nonce_vec.push(u256_to_hex(&tx.nonce));
                transaction_index_vec.push(tx.transaction_index.unwrap_or_else(U64::zero).as_u64());
                from_address_vec.push(h160_to_hex(&tx.from.unwrap_or_else(Address::zero)));
                to_address_vec.push(h160_to_hex(&tx.to.unwrap_or_else(Address::zero)));
                value_vec.push(tx.value.as_u128() as i128);
                gas_vec.push(tx.gas.as_u64());
                gas_price_vec.push(tx.gas_price.unwrap_or_else(U256::zero).as_u64());
                // Prefix with 0x
                let input = bytes_to_hex(&tx.input);
                if input.len() > 9 {
                    method_id_vec.push(input[..10].to_string());
                } else {
                    method_id_vec.push(input.to_string());
                }
                input_vec.push(bytes_to_hex(&tx.input));
                max_fee_per_gas_vec.push(tx.max_fee_per_gas.unwrap_or_else(U256::zero).as_u64());
                max_priority_fee_per_gas_vec.push(
                    tx.max_priority_fee_per_gas
                        .unwrap_or_else(U256::zero)
                        .as_u64(),
                );
                transaction_type_vec.push(tx.transaction_type.unwrap_or_else(U64::zero).as_u64());
                block_hash_vec.push(h256_to_hex(&block.hash.unwrap_or_else(H256::zero)));
                block_number_vec.push(block.number.unwrap_or_else(U64::zero).as_u64());
                block_timestamp_vec.push(block.timestamp.as_u64());
            }
        }

        // Array.
        let hash_array = Utf8Array::<i32>::from_slice(&hash_vec);
        let nonce_array = Utf8Array::<i32>::from_slice(nonce_vec);
        let transaction_index_array = UInt64Array::from_slice(transaction_index_vec);
        let from_address_array = Utf8Array::<i32>::from_slice(from_address_vec);
        let to_address_array = Utf8Array::<i32>::from_slice(to_address_vec);

        let value_array = Int128Array::from_slice(value_vec).to(DataType::Decimal(36, 18));

        let gas_array = UInt64Array::from_slice(gas_vec);
        let gas_price_array = UInt64Array::from_slice(gas_price_vec);
        let method_id_array = Utf8Array::<i32>::from_slice(method_id_vec);
        let input_array = Utf8Array::<i32>::from_slice(input_vec);
        let max_fee_per_gas_array = UInt64Array::from_slice(max_fee_per_gas_vec);
        let max_priority_fee_per_gas_array = UInt64Array::from_slice(max_priority_fee_per_gas_vec);
        let transaction_type_array = UInt64Array::from_slice(transaction_type_vec);
        let block_hash_array = Utf8Array::<i32>::from_slice(block_hash_vec);
        let block_number_array = UInt64Array::from_slice(block_number_vec);
        let block_timestamp_array = UInt64Array::from_slice(block_timestamp_vec);

        // Field.
        let hash_field = Field::new("hash", hash_array.data_type().clone(), true);
        let nonce_field = Field::new("nonce", nonce_array.data_type().clone(), true);
        let transaction_index_field = Field::new(
            "transaction_index",
            transaction_index_array.data_type().clone(),
            true,
        );
        let from_address_field =
            Field::new("from_address", from_address_array.data_type().clone(), true);
        let to_address_field = Field::new("to_address", to_address_array.data_type().clone(), true);
        let value_field = Field::new("value", value_array.data_type().clone(), true);
        let gas_field = Field::new("gas", gas_array.data_type().clone(), true);
        let gas_price_field = Field::new("gas_price", gas_price_array.data_type().clone(), true);
        let method_id_field = Field::new("method_id", method_id_array.data_type().clone(), true);
        let input_field = Field::new("input", input_array.data_type().clone(), true);
        let max_fee_per_gas_field = Field::new(
            "max_fee_per_gas",
            max_fee_per_gas_array.data_type().clone(),
            true,
        );
        let max_priority_fee_per_gas_field = Field::new(
            "max_priority_fee_per_gas",
            max_priority_fee_per_gas_array.data_type().clone(),
            true,
        );
        let transaction_type_field = Field::new(
            "transaction_type",
            transaction_type_array.data_type().clone(),
            true,
        );
        let block_hash_field = Field::new("block_hash", block_hash_array.data_type().clone(), true);
        let block_number_field =
            Field::new("block_number", block_number_array.data_type().clone(), true);
        let block_timestamp_field = Field::new(
            "block_timestamp",
            block_timestamp_array.data_type().clone(),
            true,
        );

        let schema = Schema::from(vec![
            hash_field,
            nonce_field,
            transaction_index_field,
            from_address_field,
            to_address_field,
            value_field,
            gas_field,
            gas_price_field,
            method_id_field,
            input_field,
            max_fee_per_gas_field,
            max_priority_fee_per_gas_field,
            transaction_type_field,
            block_hash_field,
            block_number_field,
            block_timestamp_field,
        ]);

        let columns = Chunk::try_new(vec![
            hash_array.boxed(),
            nonce_array.boxed(),
            transaction_index_array.boxed(),
            from_address_array.boxed(),
            to_address_array.boxed(),
            value_array.boxed(),
            gas_array.boxed(),
            gas_price_array.boxed(),
            method_id_array.boxed(),
            input_array.boxed(),
            max_fee_per_gas_array.boxed(),
            max_priority_fee_per_gas_array.boxed(),
            transaction_type_array.boxed(),
            block_hash_array.boxed(),
            block_number_array.boxed(),
            block_timestamp_array.boxed(),
        ])?;

        let tx_path = format!(
            "{}/transactions/transactions_{}",
            self.output_dir, self.range_path
        );
        write_file(&self.ctx, &tx_path, schema, columns, "transactions").await?;
        self.write_tx_hash_file(&hash_vec).await
    }

    pub async fn write_tx_hash_file(&self, tx_hashes: &[String]) -> Result<()> {
        let path = format!(
            "{}/transactions/_transactions_hash_{}.txt",
            self.output_dir, self.range_path
        );
        let mut cursor = Cursor::new(Vec::new());
        for hash in tx_hashes {
            writeln!(cursor, "{}", hash)?;
        }
        cursor.flush()?;

        log::info!("Write {}", path);
        common_storages::write_txt(self.ctx.get_storage(), &path, cursor.get_ref().as_slice()).await
    }
}
