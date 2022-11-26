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

use std::time::Duration;

use common_exceptions::Result;
use log::error;
use log::info;
use ticker::Ticker;
use web3::types::SyncState;

use crate::contexts::ContextRef;
use crate::eth::Syncing;
use crate::exporters::Worker;

static SYNCING_STATUS_FILE: &str = "mars_syncing_status.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SyncingStatus {
    start: usize,
    end: usize,
}

pub struct Stream {
    ctx: ContextRef,
}

impl Stream {
    pub fn create(ctx: ContextRef) -> Self {
        Stream { ctx }
    }

    pub async fn start(&self) -> Result<()> {
        let mut start = self.ctx.get_config().export.start_block;

        // Fetch syncing file.
        {
            let op = self.ctx.get_storage();
            if let Ok(data) = op.object(SYNCING_STATUS_FILE).read().await {
                let prev_syncing_status: SyncingStatus = serde_json::from_slice(&data)?;
                start = prev_syncing_status.end + 1;
                info!(
                    "Found syncing status file={}, status={:?}",
                    SYNCING_STATUS_FILE, prev_syncing_status
                );
            }
        }

        let ticker = Ticker::new(0.., Duration::from_secs(1));
        for _i in ticker {
            // Fetch syncing state.
            let end = {
                let syncing_state = Syncing::create(&self.ctx).fetch().await?;
                match syncing_state {
                    SyncState::Syncing(v) => {
                        let syncing_current_block = v.current_block.as_usize();
                        let syncing_highest_block = v.highest_block.as_usize();
                        info!(
                            "eth.syncing, currentBlock={:}, highestBlock={:}",
                            syncing_current_block, syncing_highest_block
                        );
                        syncing_current_block
                    }
                    SyncState::NotSyncing => {
                        error!("eth.syncing stopped, please check your eth node is working fine");
                        0usize
                    }
                }
            };
            if start <= end {
                self.syncing_batch(start, end).await?;
                start = end + 1;
            }
        }

        Ok(())
    }

    async fn syncing_batch(&self, start: usize, end: usize) -> Result<()> {
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
                op.object(SYNCING_STATUS_FILE).write(syncing_json).await?;
                info!(
                    "Syncing batch[{}], write file={}, status={:?}",
                    i, SYNCING_STATUS_FILE, syncing_status
                );
            }
        }

        Ok(())
    }
}
