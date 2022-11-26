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
use common_exceptions::Retryable;
use web3::types::SyncState;

use crate::contexts::ContextRef;

pub struct Syncing {
    ctx: ContextRef,
}

impl Syncing {
    pub fn create(ctx: &ContextRef) -> Self {
        Self { ctx: ctx.clone() }
    }

    pub async fn fetch(&self) -> Result<SyncState> {
        let notify = |e, duration| {
            log::warn!(
                "Fetch syncing api error at duration {:?}, error:{:?}",
                duration,
                e
            )
        };
        let op = || async {
            let res = self.fetch_with_no_retry().await?;
            Ok(res)
        };

        op.retry_with_notify(notify).await
    }

    // Get the blocks.
    async fn fetch_with_no_retry(&self) -> Result<SyncState> {
        let http = web3::transports::Http::new(self.ctx.get_rpc_url())?;
        let web3 = web3::Web3::new(http);

        Ok(web3.eth().syncing().await?)
    }
}
