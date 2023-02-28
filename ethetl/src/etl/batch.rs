// Copyright 2023 BohuTANG.
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
use log::info;

use crate::contexts::ContextRef;
use crate::etl::SyncingStatus;
use crate::etl::Worker;

pub struct Batch {
    ctx: ContextRef,
}

impl Batch {
    pub fn create(ctx: ContextRef) -> Self {
        Batch { ctx }
    }

    pub async fn syncing(&self, start: usize, end: usize, sync_status_file: &str) -> Result<()> {
        // Incr progress.
        self.ctx.get_progress().inc_all(end - start + 1);

        let op = self.ctx.get_storage();

        let range: Vec<usize> = (start..=end).collect();
        // Fits each chunk to max worker.
        let chunk_size = self.ctx.get_batch_size() * self.ctx.get_max_worker();
        let chunks = range.chunks(chunk_size);
        info!(
            "Syncing batch, range=[{:?}, {:?}], chunk_size={}, chunks={}",
            start,
            end,
            chunk_size,
            chunks.len()
        );

        for (i, chunk) in chunks.enumerate() {
            let syncing_status = SyncingStatus {
                start: *chunk.first().unwrap(),
                end: *chunk.last().unwrap(),
            };

            // Start to syncing.
            {
                info!("Syncing batch[{}], status={:?}", i, syncing_status);
                let worker = Worker::create(&self.ctx, chunk.to_vec());
                worker.start().await?;
            }

            // Write syncing file.
            {
                let syncing_json = serde_json::to_vec(&syncing_status)?;
                op.object(sync_status_file).write(syncing_json).await?;
                info!(
                    "Syncing batch[{}], write file={}, status={:?}",
                    i, sync_status_file, syncing_status
                );
            }
        }

        Ok(())
    }
}
