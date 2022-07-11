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

use common_exceptions::Result;
use futures::stream;
use futures::stream::TryStreamExt;
use futures::StreamExt;
use web3::types::H256;

use crate::contexts::ContextRef;
use crate::exporters::ReceiptExporter;

pub struct ReceiptWorker {
    ctx: ContextRef,
    hashes: Vec<H256>,
}

impl ReceiptWorker {
    pub fn create(ctx: &ContextRef, hashes: Vec<H256>) -> Self {
        Self {
            ctx: ctx.clone(),
            hashes,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let jobs = self.hashes.chunks(self.ctx.get_batch_size()).len();
        stream::iter(0..jobs)
            .map(Ok)
            .try_for_each_concurrent(self.ctx.get_max_worker(), |job| async move {
                let mut chunks = self.hashes.chunks(self.ctx.get_batch_size());
                if let Some(chunk) = chunks.nth(job) {
                    let export = ReceiptExporter::create(&self.ctx, chunk.to_vec());
                    export.export().await?;
                }
                Ok(())
            })
            .await
    }
}
