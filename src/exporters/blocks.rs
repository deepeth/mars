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

use futures::stream;
use futures::StreamExt;

use crate::exceptions::Result;
use crate::BlockWorker;
use crate::ContextRef;
use crate::ReceiptExporter;

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
        let jobs = self.numbers.chunks(self.ctx.get_batch_size()).len();
        stream::iter(0..jobs)
            .for_each_concurrent(self.ctx.get_max_worker(), |job| async move {
                let mut block_worker = BlockWorker::create(&self.ctx);
                let mut chunks = self.numbers.chunks(self.ctx.get_batch_size());
                let chunk = chunks.nth(job).unwrap();

                block_worker.push_batch(chunk.to_vec()).unwrap();
                let blocks = block_worker.execute().await.unwrap();

                let mut tx_hashes = vec![];
                for block in blocks {
                    for tx in block.transactions {
                        tx_hashes.push(tx.hash);
                    }
                }

                // Receipts.
                let receipt_worker = ReceiptExporter::create(&self.ctx, tx_hashes);
                receipt_worker.export().await.unwrap();
            })
            .await;
        Ok(())
    }
}
