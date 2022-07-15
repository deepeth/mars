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

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use common_exceptions::Result;
use futures::stream;
use futures::stream::TryStreamExt;
use futures::StreamExt;
use web3::types::H256;

use crate::contexts::ContextRef;
use crate::exporters::ReceiptExporter;

pub struct ReceiptWorker {
    ctx: ContextRef,
    block_numbers: Vec<usize>,
}

impl ReceiptWorker {
    pub fn create(ctx: &ContextRef, block_numbers: Vec<usize>) -> Self {
        Self {
            ctx: ctx.clone(),
            block_numbers,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let jobs = self.block_numbers.chunks(self.ctx.get_batch_size()).len();
        stream::iter(0..jobs)
            .map(Ok)
            .try_for_each_concurrent(self.ctx.get_max_worker(), |job| async move {
                let mut chunks = self.block_numbers.chunks(self.ctx.get_batch_size());

                if let Some(chunk) = chunks.nth(job) {
                    let start = chunk[0];
                    let end = chunk[chunk.len() - 1];
                    let dir = format!("{}/{}_{}", self.ctx.get_output_dir(), start, end);
                    let path = format!("{}/.transaction_hashes.txt", dir);

                    let mut tx_hashes = vec![];
                    let file = File::open(path)?;
                    let buffered = BufReader::new(file);
                    for line in buffered.lines() {
                        let line_str = &line?;
                        tx_hashes.push(H256::from_str(line_str).unwrap());
                    }
                    let exporter = ReceiptExporter::create(&self.ctx, &dir, tx_hashes);
                    exporter.export().await?;
                }
                Ok(())
            })
            .await
    }
}
