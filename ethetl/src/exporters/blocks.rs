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
use arrow2::array::Int64Array;
use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use common_exceptions::Result;

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
        let blocks_len = blocks.len();
        let mut tx_hashes = vec![];
        for block in &blocks {
            for tx in &block.transactions {
                tx_hashes.push(tx.hash);
            }
        }

        let dir = format!("{}/{}_{}", self.ctx.get_output_dir(), self.start, self.end);
        fs::create_dir_all(&dir)?;
        let block_path = format!("{}/blocks.csv", dir);

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
        let mut numbers = Vec::with_capacity(blocks_len);
        let mut hashes = Vec::with_capacity(blocks_len);
        let mut parent_hashes = Vec::with_capacity(blocks_len);
        let mut nonces = Vec::with_capacity(blocks_len);
        let mut sha3_uncles = Vec::with_capacity(blocks_len);
        let mut logs_blooms = Vec::with_capacity(blocks_len);
        let mut transactions_roots = Vec::with_capacity(blocks_len);
        let mut state_roots = Vec::with_capacity(blocks_len);
        let mut receipts_roots = Vec::with_capacity(blocks_len);
        let mut difficulty = Vec::with_capacity(blocks_len);
        let mut total_difficulty = Vec::with_capacity(blocks_len);
        let mut sizes = Vec::with_capacity(blocks_len);
        let mut extra_datas = Vec::with_capacity(blocks_len);
        let mut gas_limits = Vec::with_capacity(blocks_len);
        let mut gas_useds = Vec::with_capacity(blocks_len);
        let mut timestamps = Vec::with_capacity(blocks_len);
        let mut transaction_counts = Vec::with_capacity(blocks_len);
        let mut base_fee_per_gas = Vec::with_capacity(blocks_len);

        for block in &blocks {
            numbers.push(block.number.unwrap().as_u64() as i64);
            hashes.push(format!("{:#x}", block.hash.unwrap()));
            parent_hashes.push(format!("{:#x}", block.parent_hash));
            nonces.push(format!("{:#x}", block.nonce.unwrap()));
            sha3_uncles.push(format!("{:#x}", block.uncles_hash));
            logs_blooms.push(format!("{:#x}", block.logs_bloom.unwrap()));
            transactions_roots.push(format!("{:#x}", block.transactions_root));
            state_roots.push(format!("{:#x}", block.state_root));
            receipts_roots.push(format!("{:#x}", block.receipts_root));
            difficulty.push(format!("{:#x}", block.difficulty));
            total_difficulty.push(format!("{:#x}", block.total_difficulty.unwrap()));
            sizes.push(block.size.unwrap().as_u64());
            extra_datas.push(format!("{:x?}", block.extra_data.0));
            gas_limits.push(block.gas_limit.as_u64());
            gas_useds.push(block.gas_used.as_u64());
            timestamps.push(block.timestamp.as_u64());
            transaction_counts.push(block.transactions.len() as u64);
            base_fee_per_gas.push(block.base_fee_per_gas.unwrap().as_u64());
        }

        let number_array = Int64Array::from_slice(numbers);
        let hash_array = Utf8Array::<i32>::from_slice(hashes);
        let parent_hash_array = Utf8Array::<i32>::from_slice(parent_hashes);
        let nonce_array = Utf8Array::<i32>::from_slice(nonces);
        let sha3_uncle_array = Utf8Array::<i32>::from_slice(sha3_uncles);
        let log_bloom_array = Utf8Array::<i32>::from_slice(logs_blooms);
        let transactions_root_array = Utf8Array::<i32>::from_slice(transactions_roots);
        let state_root_array = Utf8Array::<i32>::from_slice(state_roots);
        let receipts_root_array = Utf8Array::<i32>::from_slice(receipts_roots);
        let difficulty_array = Utf8Array::<i32>::from_slice(difficulty);
        let total_difficulty_array = Utf8Array::<i32>::from_slice(total_difficulty);
        let size_array = UInt64Array::from_slice(sizes);
        let extra_data_array = Utf8Array::<i32>::from_slice(extra_datas);
        let gas_limit_array = UInt64Array::from_slice(gas_limits);
        let gas_used_array = UInt64Array::from_slice(gas_useds);
        let timestamp_array = UInt64Array::from_slice(timestamps);
        let transaction_count_array = UInt64Array::from_slice(transaction_counts);
        let base_fee_per_gas_array = UInt64Array::from_slice(base_fee_per_gas);

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

        common_formats::write_csv(&block_path, header, &[column_batch])?;

        Ok(())
    }
}
