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

use std::fs;

use arrow2::array::Array;
use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use common_exceptions::Result;
use web3::ethabi::Address;
use web3::types::Block;
use web3::types::Transaction;
use web3::types::H2048;
use web3::types::H256;
use web3::types::H64;
use web3::types::U256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::eth::BlockFetcher;

pub struct BlockExporter {
    ctx: ContextRef,
    start: usize,
    end: usize,
    numbers: Vec<usize>,
}

impl BlockExporter {
    pub fn create(ctx: &ContextRef, numbers: Vec<usize>) -> BlockExporter {
        let start = numbers[0];
        let end = numbers[numbers.len() - 1];
        Self {
            ctx: ctx.clone(),
            start,
            end,
            numbers,
        }
    }

    pub async fn export(&self) -> Result<()> {
        let mut fetcher = BlockFetcher::create(&self.ctx);
        fetcher.push_batch(self.numbers.to_vec())?;
        let blocks = fetcher.fetch().await?;
        self.export_blocks(&blocks).await?;
        self.export_txs(&blocks).await?;

        Ok(())
    }

    pub async fn export_blocks(&self, blocks: &[Block<Transaction>]) -> Result<()> {
        let blocks_len = blocks.len();

        let header = vec![
            "number",
            "hash",
            "parent_hash",
            "nonce",
            "sha3_uncles",
            "logs_bloom",
            "transactions_root",
            "state_root",
            "receipts_root",
            "difficulty",
            "total_difficulty",
            "size",
            "extra_data",
            "gas_limit",
            "gas_used",
            "timestamp",
            "transaction_count",
            "base_fee_per_gas",
        ];
        let mut number_vec = Vec::with_capacity(blocks_len);
        let mut hash_vec = Vec::with_capacity(blocks_len);
        let mut parent_hash_vec = Vec::with_capacity(blocks_len);
        let mut nonce_vec = Vec::with_capacity(blocks_len);
        let mut sha3_uncle_vec = Vec::with_capacity(blocks_len);
        let mut logs_bloom_vec = Vec::with_capacity(blocks_len);
        let mut transactions_root_vec = Vec::with_capacity(blocks_len);
        let mut state_root_vec = Vec::with_capacity(blocks_len);
        let mut receipts_root_vec = Vec::with_capacity(blocks_len);
        let mut difficulty_vec = Vec::with_capacity(blocks_len);
        let mut total_difficulty_vec = Vec::with_capacity(blocks_len);
        let mut size_vec = Vec::with_capacity(blocks_len);
        let mut extra_data_vec = Vec::with_capacity(blocks_len);
        let mut gas_limit_vec = Vec::with_capacity(blocks_len);
        let mut gas_used_vec = Vec::with_capacity(blocks_len);
        let mut timestamp_vec = Vec::with_capacity(blocks_len);
        let mut transaction_count_vec = Vec::with_capacity(blocks_len);
        let mut base_fee_per_gas_vec = Vec::with_capacity(blocks_len);

        for block in blocks {
            number_vec.push(block.number.unwrap_or_else(U64::zero).as_u64());
            hash_vec.push(format!("{:#x}", block.hash.unwrap_or_else(H256::zero)));
            parent_hash_vec.push(format!("{:#x}", block.parent_hash));
            nonce_vec.push(format!("{:#x}", block.nonce.unwrap_or_else(H64::zero)));
            sha3_uncle_vec.push(format!("{:#x}", block.uncles_hash));
            logs_bloom_vec.push(format!(
                "{:#x}",
                block.logs_bloom.unwrap_or_else(H2048::zero)
            ));
            transactions_root_vec.push(format!("{:#x}", block.transactions_root));
            state_root_vec.push(format!("{:#x}", block.state_root));
            receipts_root_vec.push(format!("{:#x}", block.receipts_root));
            difficulty_vec.push(format!("{:}", block.difficulty));
            total_difficulty_vec.push(format!(
                "{:}",
                block.total_difficulty.unwrap_or_else(U256::zero)
            ));
            size_vec.push(block.size.unwrap_or_else(U256::zero).as_u64());
            extra_data_vec.push(format!(
                "0x{}",
                block
                    .extra_data
                    .0
                    .iter()
                    .map(|x| format!("{:02x}", x))
                    .collect::<String>()
            ));
            gas_limit_vec.push(block.gas_limit.as_u64());
            gas_used_vec.push(block.gas_used.as_u64());
            timestamp_vec.push(block.timestamp.as_u64());
            transaction_count_vec.push(block.transactions.len() as u64);
            base_fee_per_gas_vec.push(block.base_fee_per_gas.unwrap_or_else(U256::zero).as_u64());
        }

        let number_array = UInt64Array::from_slice(number_vec);
        let hash_array = Utf8Array::<i32>::from_slice(hash_vec);
        let parent_hash_array = Utf8Array::<i32>::from_slice(parent_hash_vec);
        let nonce_array = Utf8Array::<i32>::from_slice(nonce_vec);
        let sha3_uncle_array = Utf8Array::<i32>::from_slice(sha3_uncle_vec);
        let log_bloom_array = Utf8Array::<i32>::from_slice(logs_bloom_vec);
        let transactions_root_array = Utf8Array::<i32>::from_slice(transactions_root_vec);
        let state_root_array = Utf8Array::<i32>::from_slice(state_root_vec);
        let receipts_root_array = Utf8Array::<i32>::from_slice(receipts_root_vec);
        let difficulty_array = Utf8Array::<i32>::from_slice(difficulty_vec);
        let total_difficulty_array = Utf8Array::<i32>::from_slice(total_difficulty_vec);
        let size_array = UInt64Array::from_slice(size_vec);
        let extra_data_array = Utf8Array::<i32>::from_slice(extra_data_vec);
        let gas_limit_array = UInt64Array::from_slice(gas_limit_vec);
        let gas_used_array = UInt64Array::from_slice(gas_used_vec);
        let timestamp_array = UInt64Array::from_slice(timestamp_vec);
        let transaction_count_array = UInt64Array::from_slice(transaction_count_vec);
        let base_fee_per_gas_array = UInt64Array::from_slice(base_fee_per_gas_vec);

        let column_batch = Chunk::try_new(vec![
            &number_array as &dyn Array,
            &hash_array as &dyn Array,
            &parent_hash_array as &dyn Array,
            &nonce_array as &dyn Array,
            &sha3_uncle_array as &dyn Array,
            &log_bloom_array as &dyn Array,
            &transactions_root_array as &dyn Array,
            &state_root_array as &dyn Array,
            &receipts_root_array as &dyn Array,
            &difficulty_array as &dyn Array,
            &total_difficulty_array as &dyn Array,
            &size_array as &dyn Array,
            &extra_data_array as &dyn Array,
            &gas_limit_array as &dyn Array,
            &gas_used_array as &dyn Array,
            &timestamp_array as &dyn Array,
            &transaction_count_array as &dyn Array,
            &base_fee_per_gas_array as &dyn Array,
        ])?;

        let dir = format!("{}/{}_{}", self.ctx.get_output_dir(), self.start, self.end);
        fs::create_dir_all(&dir)?;
        let block_path = format!("{}/blocks.csv", dir);
        common_formats::write_csv(&block_path, header, &[column_batch])
    }

    pub async fn export_txs(&self, blocks: &[Block<Transaction>]) -> Result<()> {
        let header = vec![
            "hash",
            "nonce",
            "transaction_index",
            "form_address",
            "to_address",
            "value",
            "gas",
            "gas_price",
            "input",
            "max_fee_per_gas",
            "max_priority_fee_per_gas",
            "transaction_type",
            "block_hash",
            "block_number",
            "block_timestamp",
        ];

        let mut hash_vec = vec![];
        let mut nonce_vec = vec![];
        let mut transaction_index_vec = vec![];
        let mut from_address_vec = vec![];
        let mut to_address_vec = vec![];
        let mut value_vec = vec![];
        let mut gas_vec = vec![];
        let mut gas_price_vec = vec![];
        let mut input_vec = vec![];
        let mut max_fee_per_gas_vec = vec![];
        let mut max_priority_fee_per_gas_vec = vec![];
        let mut transaction_type_vec = vec![];
        let mut block_hash_vec = vec![];
        let mut block_number_vec = vec![];
        let mut block_timestamp_vec = vec![];

        for block in blocks {
            for tx in &block.transactions {
                hash_vec.push(format!("{:#x}", tx.hash));
                nonce_vec.push(format!("{:}", tx.nonce));
                transaction_index_vec.push(tx.transaction_index.unwrap_or_else(U64::zero).as_u64());
                from_address_vec.push(format!("{:#x}", tx.from.unwrap_or_else(Address::zero)));
                to_address_vec.push(format!("{:#x}", tx.to.unwrap_or_else(Address::zero)));
                value_vec.push(format!("{:}", tx.value));
                gas_vec.push(format!("{:}", tx.gas));
                gas_price_vec.push(format!("{:}", tx.gas_price.unwrap_or_else(U256::zero)));
                input_vec.push(format!(
                    "0x{}",
                    tx.input
                        .0
                        .iter()
                        .map(|x| format!("{:02x}", x))
                        .collect::<String>()
                ));
                max_fee_per_gas_vec.push(tx.max_fee_per_gas.unwrap_or_else(U256::zero).as_u64());
                max_priority_fee_per_gas_vec.push(
                    tx.max_priority_fee_per_gas
                        .unwrap_or_else(U256::zero)
                        .as_u64(),
                );
                transaction_type_vec.push(tx.transaction_type.unwrap_or_else(U64::zero).as_u64());
                block_hash_vec.push(format!("{:#x}", block.hash.unwrap_or_else(H256::zero)));
                block_number_vec.push(block.number.unwrap_or_else(U64::zero).as_u64());
                block_timestamp_vec.push(block.timestamp.as_u64());
            }
        }

        let hash_array = Utf8Array::<i32>::from_slice(hash_vec);
        let nonce_array = Utf8Array::<i32>::from_slice(nonce_vec);
        let transaction_index_array = UInt64Array::from_slice(transaction_index_vec);
        let from_address_array = Utf8Array::<i32>::from_slice(from_address_vec);
        let to_address_array = Utf8Array::<i32>::from_slice(to_address_vec);
        let value_array = Utf8Array::<i32>::from_slice(value_vec);
        let gas_array = Utf8Array::<i32>::from_slice(gas_vec);
        let gas_price_array = Utf8Array::<i32>::from_slice(gas_price_vec);
        let input_array = Utf8Array::<i32>::from_slice(input_vec);
        let max_fee_per_gas_array = UInt64Array::from_slice(max_fee_per_gas_vec);
        let max_priority_fee_per_gas_array = UInt64Array::from_slice(max_priority_fee_per_gas_vec);
        let transaction_type_array = UInt64Array::from_slice(transaction_type_vec);
        let block_hash_array = Utf8Array::<i32>::from_slice(block_hash_vec);
        let block_number_array = UInt64Array::from_slice(block_number_vec);
        let block_timestamp_array = UInt64Array::from_slice(block_timestamp_vec);

        let column_batch = Chunk::try_new(vec![
            &hash_array as &dyn Array,
            &nonce_array as &dyn Array,
            &transaction_index_array as &dyn Array,
            &from_address_array as &dyn Array,
            &to_address_array as &dyn Array,
            &value_array as &dyn Array,
            &gas_array as &dyn Array,
            &gas_price_array as &dyn Array,
            &input_array as &dyn Array,
            &max_fee_per_gas_array as &dyn Array,
            &max_priority_fee_per_gas_array as &dyn Array,
            &transaction_type_array as &dyn Array,
            &block_hash_array as &dyn Array,
            &block_number_array as &dyn Array,
            &block_timestamp_array as &dyn Array,
        ])?;

        let dir = format!("{}/{}_{}", self.ctx.get_output_dir(), self.start, self.end);
        fs::create_dir_all(&dir)?;
        let tx_path = format!("{}/transactions.csv", dir);
        common_formats::write_csv(&tx_path, header, &[column_batch])
    }
}
