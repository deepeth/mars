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
use crate::etl::Batch;
use crate::etl::SyncingStatus;
use crate::etl::SYNCING_STATUS_FILE;

pub struct NormalEtl {
    ctx: ContextRef,
}

impl NormalEtl {
    pub fn create(ctx: ContextRef) -> Self {
        NormalEtl { ctx }
    }

    pub async fn start(&self) -> Result<()> {
        let mut start = self.ctx.get_config().export.start_block;
        let end = self.ctx.get_config().export.end_block;

        // Fetch syncing file.
        {
            let op = self.ctx.get_storage();
            if let Ok(data) = op.object(SYNCING_STATUS_FILE).read().await {
                let prev_syncing_status: SyncingStatus = serde_json::from_slice(&data)?;
                start = prev_syncing_status.end + 1;
                info!(
                    "Found normal syncing status file={}, status={:?}",
                    SYNCING_STATUS_FILE, prev_syncing_status
                );
            }
        }

        if start <= end {
            let batch = Batch::create(self.ctx.clone());
            batch.syncing(start, end, SYNCING_STATUS_FILE).await?;
        }

        Ok(())
    }
}
