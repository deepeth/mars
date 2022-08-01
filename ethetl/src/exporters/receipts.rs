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
use web3::types::Address;
use web3::types::TransactionReceipt;
use web3::types::H256;
use web3::types::U256;
use web3::types::U64;

use crate::contexts::ContextRef;
use crate::eth::ReceiptFetcher;
use crate::exporters::write_file;
use crate::exporters::TokenTransferExporter;

pub struct ReceiptExporter {
    ctx: ContextRef,
    dir: String,
    hashes: Vec<H256>,
}

impl ReceiptExporter {
    pub fn create(ctx: &ContextRef, dir: &str, hashes: Vec<H256>) -> ReceiptExporter {
        Self {
            ctx: ctx.clone(),
            dir: dir.to_string(),
            hashes,
        }
    }

    pub async fn export(&self) -> Result<()> {
        let mut fetcher = ReceiptFetcher::create(&self.ctx);
        fetcher.push_batch(self.hashes.to_vec())?;

        // Receipts.
        let receipts = fetcher.fetch().await?;
        self.export_receipts(&receipts).await?;

        // Token transfers.
        let token_transfer_export = TokenTransferExporter::create(&self.ctx, &self.dir, &receipts);
        token_transfer_export.export().await
    }

    pub async fn export_receipts(&self, receipts: &[TransactionReceipt]) -> Result<()> {
        let receipt_len = receipts.len();
        let mut transaction_hash_vec = Vec::with_capacity(receipt_len);
        let mut transaction_index_vec = Vec::with_capacity(receipt_len);
        let mut block_hash_vec = Vec::with_capacity(receipt_len);
        let mut block_number_vec = Vec::with_capacity(receipt_len);
        let mut cumulative_gas_used_vec = Vec::with_capacity(receipt_len);
        let mut gas_used_vec = Vec::with_capacity(receipt_len);
        let mut contract_address_vec = Vec::with_capacity(receipt_len);
        let mut root_vec = Vec::with_capacity(receipt_len);
        let mut status_vec = Vec::with_capacity(receipt_len);
        let mut effective_gas_price_vec = Vec::with_capacity(receipt_len);

        for receipt in receipts {
            transaction_hash_vec.push(format!("{:#x}", receipt.transaction_hash));
            transaction_index_vec.push(receipt.transaction_index.as_u64());
            block_hash_vec.push(format!(
                "{:#x}",
                receipt.block_hash.unwrap_or_else(H256::zero)
            ));
            block_number_vec.push(receipt.block_number.unwrap_or_else(U64::zero).as_u64());
            cumulative_gas_used_vec.push(receipt.cumulative_gas_used.as_u64());
            gas_used_vec.push(receipt.gas_used.unwrap_or_else(U256::zero).as_u64());
            contract_address_vec.push(format!(
                "{:#x}",
                receipt.contract_address.unwrap_or_else(Address::zero)
            ));
            root_vec.push(format!("{:#x}", receipt.root.unwrap_or_else(H256::zero)));
            status_vec.push(receipt.status.unwrap_or_else(U64::zero).as_u64());
            effective_gas_price_vec.push(
                receipt
                    .effective_gas_price
                    .unwrap_or_else(U256::zero)
                    .as_u64(),
            );
        }
        let transaction_hash_array = Utf8Array::<i32>::from_slice(transaction_hash_vec);
        let transaction_index_array = UInt64Array::from_slice(transaction_index_vec);
        let block_hash_array = Utf8Array::<i32>::from_slice(block_hash_vec);
        let block_number_array = UInt64Array::from_slice(block_number_vec);
        let cumulative_gas_used_array = UInt64Array::from_slice(cumulative_gas_used_vec);
        let gas_used_array = UInt64Array::from_slice(gas_used_vec);
        let contract_address_array = Utf8Array::<i32>::from_slice(contract_address_vec);
        let root_array = Utf8Array::<i32>::from_slice(root_vec);
        let status_array = UInt64Array::from_slice(status_vec);
        let effective_gas_price_array = UInt64Array::from_slice(effective_gas_price_vec);

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
        let cumulative_gas_used_field = Field::new(
            "cumulative_gas_used",
            cumulative_gas_used_array.data_type().clone(),
            true,
        );
        let gas_used_field = Field::new("gas_used", gas_used_array.data_type().clone(), true);
        let contract_address_field = Field::new(
            "contract_address",
            contract_address_array.data_type().clone(),
            true,
        );
        let root_field = Field::new("root", root_array.data_type().clone(), true);
        let status_field = Field::new("status", status_array.data_type().clone(), true);
        let effective_gas_price_field = Field::new(
            "effective_gas_price",
            effective_gas_price_array.data_type().clone(),
            true,
        );

        let schema = Schema::from(vec![
            transaction_hash_field,
            transaction_index_field,
            block_hash_field,
            block_number_field,
            cumulative_gas_used_field,
            gas_used_field,
            contract_address_field,
            root_field,
            status_field,
            effective_gas_price_field,
        ]);
        let columns = Chunk::try_new(vec![
            transaction_hash_array.boxed(),
            transaction_index_array.boxed(),
            block_hash_array.boxed(),
            block_number_array.boxed(),
            cumulative_gas_used_array.boxed(),
            gas_used_array.boxed(),
            contract_address_array.boxed(),
            root_array.boxed(),
            status_array.boxed(),
            effective_gas_price_array.boxed(),
        ])?;

        let receipt_path = format!("{}/receipts", self.dir);
        write_file(&self.ctx, &receipt_path, schema, columns, "receipts").await
    }
}
