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
use futures::stream::TryStreamExt;
use futures::StreamExt;
use web3::types::H256;

use crate::exceptions::Result;
use crate::ContextRef;
use crate::ReceiptWorker;

pub struct ReceiptExporter {
    ctx: ContextRef,
    hashes: Vec<H256>,
}

impl ReceiptExporter {
    pub fn create(ctx: &ContextRef, hashes: Vec<H256>) -> ReceiptExporter {
        Self {
            ctx: ctx.clone(),
            hashes,
        }
    }

    pub async fn export(&self) -> Result<()> {
        let jobs = self.hashes.chunks(self.ctx.get_batch_size()).len();
        stream::iter(0..jobs)
            .map(Ok)
            .try_for_each_concurrent(self.ctx.get_max_worker(), |job| async move {
                let mut worker = ReceiptWorker::create(&self.ctx);
                let mut chunks = self.hashes.chunks(self.ctx.get_batch_size());
                let chunk = chunks.nth(job).unwrap();

                worker.push_batch(chunk.to_vec()).unwrap();
                worker.execute().await?;
                Ok(())
            })
            .await
    }
}
