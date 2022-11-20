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
use log::info;

use crate::contexts::ContextRef;
use crate::eth::BlockNumber;
use crate::exporters::Worker;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct StreamSyncing {
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
        // Try to read stream_syncing.json
        let stream_syncing_file = "stream_syncing.json";
        let op = self.ctx.get_storage();
        let mut start = self.ctx.get_config().export.start_block;

        let block_number = BlockNumber::create(&self.ctx);
        let end = block_number.fetch().await?.as_u64() as usize;
        info!("Stream chain latest block is: {:?}", end);

        // Check syncing file.
        if let Ok(data) = op.object(stream_syncing_file).read().await {
            let prev_syncing: StreamSyncing = serde_json::from_slice(&data)?;
            start = prev_syncing.end + 1;
            info!(
                "Found stream syncing file, syncing status: {:?}, change the start block to:{}",
                prev_syncing, start
            );
        }

        info!("Stream sync range: [{:?}, {:?}]", start, end);
        let range: Vec<usize> = (start..end + 1).collect();
        // Fits each chunk to max worker.
        let chunk_size = self.ctx.get_batch_size() * self.ctx.get_max_worker();
        for chunk in range.chunks(chunk_size) {
            let syncing = StreamSyncing {
                start: *chunk.first().unwrap(),
                end: *chunk.last().unwrap(),
            };
            info!("Stream syncing chunk: {:?}", syncing);
            let worker = Worker::create(&self.ctx, chunk.to_vec());
            worker.start().await?;
            // Write syncing file.
        }

        Ok(())
    }
}
