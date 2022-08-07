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
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use web3::types::Block;
use web3::types::BlockNumber;
use web3::types::Transaction;
use web3::types::U64;

use crate::contexts::ContextRef;

pub struct BlockFetcher {
    ctx: ContextRef,
    numbers: Vec<usize>,
}

impl BlockFetcher {
    pub fn create(ctx: &ContextRef) -> BlockFetcher {
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

    pub async fn fetch(&self) -> Result<Vec<Block<Transaction>>> {
        let notify = |e, duration| {
            log::warn!(
                "Fetch blocks error at duration {:?}, error:{:?}",
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
    async fn fetch_with_no_retry(&self) -> Result<Vec<Block<Transaction>>> {
        let transport = HttpClientBuilder::default()
            .build(self.ctx.get_rpc_url())
            .unwrap();
        let batch = self
            .numbers
            .iter()
            .cloned()
            .map(|x| {
                ("eth_getBlockByNumber", rpc_params![
                    serde_json::to_value(BlockNumber::Number(U64::from(x))).unwrap(),
                    serde_json::to_value(true).unwrap()
                ])
            })
            .collect::<Vec<_>>();
        Ok(transport.batch_request::<Block<Transaction>>(batch).await?)
    }
}
