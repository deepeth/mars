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
use web3::types::Address;
use web3::types::Bytes;

use crate::contexts::ContextRef;

pub struct ContractFetcher {
    ctx: ContextRef,
    addresses: Vec<Address>,
}

impl ContractFetcher {
    pub fn create(ctx: &ContextRef) -> ContractFetcher {
        Self {
            ctx: ctx.clone(),
            addresses: vec![],
        }
    }

    pub fn push(&mut self, addr: Address) -> Result<()> {
        self.addresses.push(addr);
        Ok(())
    }

    pub fn push_batch(&mut self, addrs: Vec<Address>) -> Result<()> {
        self.addresses.extend(addrs);
        Ok(())
    }

    pub async fn fetch(&self) -> Result<Vec<Bytes>> {
        let notify = |e, duration| {
            log::warn!(
                "Fetch contracts bytes error at duration {:?}, error:{:?}",
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

    async fn fetch_with_no_retry(&self) -> Result<Vec<Bytes>> {
        let http = web3::transports::Http::new(self.ctx.get_rpc_url())?;
        let web3 = web3::Web3::new(web3::transports::Batch::new(http));

        let mut contracts = vec![];

        for chunks in self.addresses.chunks(self.ctx.get_web3_batch_size()) {
            let mut callbacks = vec![];
            for addr in chunks {
                let receipt = web3.eth().code(*addr, None);
                callbacks.push(receipt);
            }
            let _ = web3.transport().submit_batch().await?;

            for cb in callbacks {
                let r = cb.await?;
                contracts.push(r);
            }
        }

        Ok(contracts)
    }
}
