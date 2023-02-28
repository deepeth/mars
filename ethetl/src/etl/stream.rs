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

use std::time::Duration;

use common_exceptions::Result;
use log::info;
use ticker::Ticker;

use crate::chains::eth::BlockNumber;
use crate::contexts::ContextRef;
use crate::etl::Batch;
use crate::etl::SyncingStatus;

static STREAM_SYNCING_STATUS_FILE: &str = "mars_stream_syncing_status.json";

pub struct StreamEtl {
    ctx: ContextRef,
}

impl StreamEtl {
    pub fn create(ctx: ContextRef) -> Self {
        StreamEtl { ctx }
    }

    pub async fn start(&self) -> Result<()> {
        let mut start = self.ctx.get_config().export.start_block;

        // Fetch syncing file.
        {
            let op = self.ctx.get_storage();
            if let Ok(data) = op.object(STREAM_SYNCING_STATUS_FILE).read().await {
                let prev_syncing_status: SyncingStatus = serde_json::from_slice(&data)?;
                start = prev_syncing_status.end + 1;
                info!(
                    "Found syncing status file={}, status={:?}",
                    STREAM_SYNCING_STATUS_FILE, prev_syncing_status
                );
            }
        }

        let ticker = Ticker::new(
            0..,
            Duration::from_secs(self.ctx.get_config().export.syncing_interval_secs as u64),
        );
        for _i in ticker {
            // Fetch syncing state.
            let end = {
                let latest_block = BlockNumber::create(&self.ctx).fetch().await?;
                info!("Eth node last block number :{}", latest_block);
                latest_block.as_usize()
            };
            if start <= end {
                let batch = Batch::create(self.ctx.clone());
                batch
                    .syncing(start, end, STREAM_SYNCING_STATUS_FILE)
                    .await?;
                start = end + 1;
            }
        }

        Ok(())
    }
}
