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

use arrow2::array::Array;
use arrow2::array::Int64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use arrow2::io::csv::write;

use crate::contexts::ContextRef;
use crate::eth::BlockFetcher;
use crate::exceptions::Result;

pub struct BlockExporter {
    ctx: ContextRef,
    numbers: Vec<usize>,
}

impl BlockExporter {
    pub fn create(ctx: &ContextRef, numbers: Vec<usize>) -> BlockExporter {
        Self {
            ctx: ctx.clone(),
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

        let path = format!("{}/blocks.csv", self.ctx.get_output_dir());
        let header = vec!["number", "hash"];
        let mut numbers = Vec::with_capacity(blocks_len);
        let mut hashes = Vec::with_capacity(blocks_len);
        for block in &blocks {
            numbers.push(block.number.unwrap().as_u64() as i64);
            hashes.push(block.hash.unwrap().to_string());
        }

        let number_array = Int64Array::from_slice(numbers);
        let hash_array = Utf8Array::<i32>::from_slice(hashes);
        let batch = Chunk::try_new(vec![&number_array as &dyn Array, &hash_array as &dyn Array])?;
        Self::write_batch(&path, header, &[batch])?;

        Ok(())
    }

    fn write_batch<A: AsRef<dyn Array>>(
        path: &str,
        headers: Vec<&str>,
        columns: &[Chunk<A>],
    ) -> Result<()> {
        let mut writer = std::fs::File::create(path)?;

        let options = write::SerializeOptions::default();
        write::write_header(&mut writer, headers.as_slice(), &options)?;

        columns
            .iter()
            .try_for_each(|batch| write::write_chunk(&mut writer, batch, &options))?;
        Ok(())
    }
}
