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
use futures::stream::StreamExt;
use futures::stream::TryStreamExt;

use crate::contexts::ContextRef;
use crate::exceptions::Result;
use crate::exporters::BlockExporter;

pub struct BlockWorker {
    ctx: ContextRef,
    numbers: Vec<usize>,
}

impl BlockWorker {
    pub fn create(ctx: &ContextRef, numbers: Vec<usize>) -> Self {
        Self {
            ctx: ctx.clone(),
            numbers,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let jobs = self.numbers.chunks(self.ctx.get_batch_size()).len();
        stream::iter(0..jobs)
            .map(Ok)
            .try_for_each_concurrent(self.ctx.get_max_worker(), |job| async move {
                let mut chunks = self.numbers.chunks(self.ctx.get_batch_size());

                if let Some(chunk) = chunks.nth(job) {
                    let export = BlockExporter::create(&self.ctx, chunk.to_vec());
                    export.export().await?;
                }
                Ok(())
            })
            .await
    }
}
