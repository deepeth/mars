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

mod blocks;
mod nft_transfers;
mod pipeline;
mod receipts;
mod token_transfers;
mod transactions;
mod worker;

use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
pub use blocks::BlockExporter;
use common_exceptions::ErrorCode;
use common_exceptions::Result;
pub use nft_transfers::NftTransferExporter;
pub use pipeline::Pipeline;
pub use receipts::ReceiptExporter;
pub use token_transfers::TokenTransferExporter;
pub use transactions::TransactionExporter;
use web3::types::Bytes;
use web3::types::H256;
pub use worker::Worker;

use crate::contexts::ContextRef;

pub const TOKEN_TRANSFER_CONTRACT_ADDRESS_HEX: &str =
    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

pub fn h256_to_hex(v: &H256) -> String {
    let hex = v
        .as_bytes()
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    hex.trim_start_matches('0').to_string()
}

pub fn bytes_to_hex(v: &Bytes) -> String {
    v.0.iter().map(|x| format!("{:02x}", x)).collect::<String>()
}

pub async fn write_file(
    ctx: &ContextRef,
    path: &str,
    schema: Schema,
    columns: Chunk<Box<dyn Array>>,
    msg: &str,
) -> Result<()> {
    match ctx.get_output_format().to_lowercase().as_str() {
        "csv" => {
            let path = format!("{}.csv", path);
            log::info!("Write {} to {}", msg, path);
            common_storages::write_csv(ctx.get_storage(), &path, schema, columns).await
        }
        "parquet" => {
            let path = format!("{}.parquet", path);
            log::info!("Write {} to {}", msg, path);
            common_storages::write_parquet(ctx.get_storage(), &path, schema, columns).await
        }
        v => Err(ErrorCode::Invalid(format!(
            "Unsupported format, must be one of [csv, parquet], got: {}",
            v
        ))),
    }
}
