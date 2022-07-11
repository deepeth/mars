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

use common_exceptions::ErrorCode;
use common_exceptions::Result;
use web3::types::TransactionReceipt;
use web3::types::H256;

use crate::contexts::ContextRef;

pub struct ReceiptFetcher {
    ctx: ContextRef,
    hashes: Vec<H256>,
}

impl ReceiptFetcher {
    pub fn create(ctx: &ContextRef) -> ReceiptFetcher {
        Self {
            ctx: ctx.clone(),
            hashes: vec![],
        }
    }

    pub fn push(&mut self, hash: H256) -> Result<()> {
        self.hashes.push(hash);
        Ok(())
    }

    pub fn push_batch(&mut self, hashes: Vec<H256>) -> Result<()> {
        self.hashes.extend(hashes);
        Ok(())
    }

    #[tracing::instrument(level = "info", skip(self))]
    pub async fn fetch(&self) -> Result<Vec<TransactionReceipt>> {
        let http = web3::transports::Http::new(self.ctx.get_rpc_url())?;
        let web3 = web3::Web3::new(web3::transports::Batch::new(http));

        let mut receipts = vec![];

        for chunks in self.hashes.chunks(1000) {
            let mut callbacks = vec![];
            for hash in chunks {
                let receipt = web3.eth().transaction_receipt(*hash);
                callbacks.push(receipt);
            }
            let _ = web3.transport().submit_batch().await?;

            for cb in callbacks {
                let r = cb.await?;
                match r {
                    None => return Err(ErrorCode::ExportReceiptError("Cannot get receipt")),
                    Some(v) => {
                        receipts.push(v);
                    }
                }
                self.ctx.get_progress().incr_receipts(1);
            }
        }

        Ok(receipts)
    }
}
