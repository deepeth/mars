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
use web3::types::BlockNumber;
use web3::types::Trace;
use web3::types::U64;

use crate::contexts::ContextRef;

pub struct Traces {
    ctx: ContextRef,
    numbers: Vec<usize>,
}

impl Traces {
    pub fn create(ctx: &ContextRef) -> Traces {
        Self {
            ctx: ctx.clone(),
            numbers: vec![],
        }
    }

    // Push a block number.
    pub fn push(&mut self, number: usize) -> Result<()> {
        self.numbers.push(number);
        Ok(())
    }

    // Push range of block numbers.
    pub fn push_batch(&mut self, nums: Vec<usize>) -> Result<()> {
        self.numbers.extend(nums);
        Ok(())
    }

    pub async fn fetch(&self) -> Result<Vec<Trace>> {
        let notify = |e, duration| {
            log::warn!(
                "Fetch traces error at duration {:?}, error:{:?}",
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

    // Get the blocks traces.
    async fn fetch_with_no_retry(&self) -> Result<Vec<Trace>> {
        let http = web3::transports::Http::new(self.ctx.get_rpc_url())?;
        let web3 = web3::Web3::new(web3::transports::Batch::new(http));

        let mut block_traces = vec![];

        for chunks in self.numbers.chunks(self.ctx.get_web3_batch_size()) {
            let mut callbacks = vec![];
            for num in chunks {
                let block_trace = web3.trace().block(BlockNumber::Number(U64::from(*num)));
                callbacks.push(block_trace);
            }
            let _ = web3.transport().submit_batch().await?;

            // Get the callback.
            for cb in callbacks {
                let r = cb.await?;
                block_traces.extend(r);
            }
        }

        Ok(block_traces)
    }
}
