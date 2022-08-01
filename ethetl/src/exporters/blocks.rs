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

use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Write;
use std::str::FromStr;

use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Field;
use arrow2::datatypes::Schema;
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
use crate::exporters::bytes_to_hex;
use crate::exporters::write_file;
use crate::exporters::ReceiptExporter;

pub struct BlockExporter {
    ctx: ContextRef,
    dir: String,
    numbers: Vec<usize>,
}

impl BlockExporter {
    pub fn create(ctx: &ContextRef, dir: &str, numbers: Vec<usize>) -> BlockExporter {
        Self {
            ctx: ctx.clone(),
            dir: dir.to_string(),
            numbers,
        }
    }

    pub async fn export(&self) -> Result<()> {
        let mut fetcher = BlockFetcher::create(&self.ctx);
        fetcher.push_batch(self.numbers.to_vec())?;
        let blocks = fetcher.fetch().await?;

        self.write_begin_file().await?;
        {
            self.export_blocks(&blocks).await?;
            self.export_txs(&blocks).await?;
            self.export_tx_receipts().await?;
        }
        self.write_commit_file().await?;

        Ok(())
    }

    pub async fn export_blocks(&self, blocks: &[Block<Transaction>]) -> Result<()> {
        let blocks_len = blocks.len();

        let mut number_vec = Vec::with_capacity(blocks_len);
        let mut hash_vec = Vec::with_capacity(blocks_len);
        let mut parent_hash_vec = Vec::with_capacity(blocks_len);
        let mut nonce_vec = Vec::with_capacity(blocks_len);
        let mut sha3_uncles_vec = Vec::with_capacity(blocks_len);
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
            sha3_uncles_vec.push(format!("{:#x}", block.uncles_hash));
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
        let sha3_uncles_array = Utf8Array::<i32>::from_slice(sha3_uncles_vec);
        let logs_bloom_array = Utf8Array::<i32>::from_slice(logs_bloom_vec);
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

        let number_field = Field::new("number", number_array.data_type().clone(), true);
        let hash_field = Field::new("hash", hash_array.data_type().clone(), true);
        let parent_hash_field =
            Field::new("parent_hash", parent_hash_array.data_type().clone(), true);
        let nonce_field = Field::new("nonce", nonce_array.data_type().clone(), true);
        let sha3_uncles_field =
            Field::new("sha3_uncles", sha3_uncles_array.data_type().clone(), true);
        let logs_bloom_field = Field::new("logs_bloom", logs_bloom_array.data_type().clone(), true);
        let transactions_root_field = Field::new(
            "transactions_root",
            transactions_root_array.data_type().clone(),
            true,
        );
        let state_root_field = Field::new("state_root", state_root_array.data_type().clone(), true);
        let receipts_root_field =
            Field::new("receipts_root", state_root_array.data_type().clone(), true);
        let difficulty_field = Field::new("difficulty", difficulty_array.data_type().clone(), true);
        let total_difficulty_field = Field::new(
            "total_difficulty",
            total_difficulty_array.data_type().clone(),
            true,
        );
        let size_field = Field::new("size", size_array.data_type().clone(), true);
        let extra_data_field = Field::new("extra_data", extra_data_array.data_type().clone(), true);
        let gas_limit_field = Field::new("gas_limit", gas_limit_array.data_type().clone(), true);
        let gas_used_field = Field::new("gas_used", gas_used_array.data_type().clone(), true);
        let timestamp_field = Field::new("timestamp", timestamp_array.data_type().clone(), true);
        let transaction_count_field = Field::new(
            "transaction_count",
            transaction_count_array.data_type().clone(),
            true,
        );
        let base_fee_per_gas_array_field = Field::new(
            "base_fee_per_gas",
            base_fee_per_gas_array.data_type().clone(),
            true,
        );

        let schema = Schema::from(vec![
            number_field,
            hash_field,
            parent_hash_field,
            nonce_field,
            sha3_uncles_field,
            logs_bloom_field,
            transactions_root_field,
            state_root_field,
            receipts_root_field,
            difficulty_field,
            total_difficulty_field,
            size_field,
            extra_data_field,
            gas_limit_field,
            gas_used_field,
            timestamp_field,
            transaction_count_field,
            base_fee_per_gas_array_field,
        ]);

        let columns = Chunk::try_new(vec![
            number_array.boxed(),
            hash_array.boxed(),
            parent_hash_array.boxed(),
            nonce_array.boxed(),
            sha3_uncles_array.boxed(),
            logs_bloom_array.boxed(),
            transactions_root_array.boxed(),
            state_root_array.boxed(),
            receipts_root_array.boxed(),
            difficulty_array.boxed(),
            total_difficulty_array.boxed(),
            size_array.boxed(),
            extra_data_array.boxed(),
            gas_limit_array.boxed(),
            gas_used_array.boxed(),
            timestamp_array.boxed(),
            transaction_count_array.boxed(),
            base_fee_per_gas_array.boxed(),
        ])?;

        let block_path = format!("{}/blocks", self.dir);
        write_file(&self.ctx, &block_path, schema, columns, "blocks").await
    }

    pub async fn export_txs(&self, blocks: &[Block<Transaction>]) -> Result<()> {
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
                gas_vec.push(tx.gas.as_u64());
                gas_price_vec.push(tx.gas_price.unwrap_or_else(U256::zero).as_u64());
                input_vec.push(format!("0x{}", bytes_to_hex(&tx.input)));
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

        // Array.
        let hash_array = Utf8Array::<i32>::from_slice(&hash_vec);
        let nonce_array = Utf8Array::<i32>::from_slice(nonce_vec);
        let transaction_index_array = UInt64Array::from_slice(transaction_index_vec);
        let from_address_array = Utf8Array::<i32>::from_slice(from_address_vec);
        let to_address_array = Utf8Array::<i32>::from_slice(to_address_vec);
        let value_array = Utf8Array::<i32>::from_slice(value_vec);
        let gas_array = UInt64Array::from_slice(gas_vec);
        let gas_price_array = UInt64Array::from_slice(gas_price_vec);
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
            input_array.boxed(),
            max_fee_per_gas_array.boxed(),
            max_priority_fee_per_gas_array.boxed(),
            transaction_type_array.boxed(),
            block_hash_array.boxed(),
            block_number_array.boxed(),
            block_timestamp_array.boxed(),
        ])?;

        let tx_path = format!("{}/transactions", self.dir);
        write_file(&self.ctx, &tx_path, schema, columns, "transactions").await?;
        self.write_tx_hash_file(&hash_vec).await
    }

    pub async fn export_tx_receipts(&self) -> Result<()> {
        let tx_hashes = self.read_tx_hash_file().await?;
        let exporter = ReceiptExporter::create(&self.ctx, &self.dir, tx_hashes);
        exporter.export().await?;
        Ok(())
    }

    pub async fn read_tx_hash_file(&self) -> Result<Vec<H256>> {
        let mut tx_hashes = vec![];
        let path = format!("{}/_transaction_hashes.txt", self.dir);

        let meta = self.ctx.get_storage().object(&path).metadata().await?;
        if meta.content_length() > 0 {
            let content = self.ctx.get_storage().object(&path).read().await?;
            let cursor = Cursor::new(content);
            let buffered = BufReader::new(cursor);

            for line in buffered.lines() {
                let line_str = &line?;
                tx_hashes.push(H256::from_str(line_str).unwrap());
            }
        }
        Ok(tx_hashes)
    }

    pub async fn write_tx_hash_file(&self, tx_hashes: &[String]) -> Result<()> {
        let path = format!("{}/_transaction_hashes.txt", self.dir);
        let mut cursor = Cursor::new(Vec::new());
        for hash in tx_hashes {
            writeln!(cursor, "{}", hash)?;
        }
        cursor.flush()?;

        log::info!("Write {}", path);
        common_storages::write_txt(self.ctx.get_storage(), &path, cursor.get_ref().as_slice()).await
    }

    pub async fn write_begin_file(&self) -> Result<()> {
        let path = format!("{}/_begin.txt", self.dir);
        let mut cursor = Cursor::new(Vec::new());
        writeln!(cursor, "begin")?;
        cursor.flush()?;
        common_storages::write_txt(self.ctx.get_storage(), &path, cursor.get_ref().as_slice()).await
    }

    pub async fn write_commit_file(&self) -> Result<()> {
        let path = format!("{}/_commit.txt", self.dir);
        let mut cursor = Cursor::new(Vec::new());
        writeln!(cursor, "commit")?;
        cursor.flush()?;
        common_storages::write_txt(self.ctx.get_storage(), &path, cursor.get_ref().as_slice()).await
    }
}
