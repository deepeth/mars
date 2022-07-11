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
use web3::types::H256;

use crate::contexts::ContextRef;
use crate::eth::ReceiptFetcher;

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
        let mut fetcher = ReceiptFetcher::create(&self.ctx);
        fetcher.push_batch(self.hashes.to_vec())?;
        fetcher.fetch().await?;
        Ok(())
    }
}
