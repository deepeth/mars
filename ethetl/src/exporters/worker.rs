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

use std::sync::Arc;

use common_exceptions::Result;
use deadqueue::unlimited::Queue;

use crate::contexts::ContextRef;
use crate::exporters::Pipeline;

pub struct Worker {
    ctx: ContextRef,
    block_numbers: Vec<usize>,
}

impl Worker {
    pub fn create(ctx: &ContextRef, block_numbers: Vec<usize>) -> Self {
        Self {
            ctx: ctx.clone(),
            block_numbers,
        }
    }

    pub async fn start(&self) -> Result<()> {
        let queue: Arc<Queue<Vec<usize>>> = Arc::new(Queue::new());
        let chunks = self.block_numbers.chunks(self.ctx.get_batch_size());
        for chunk in chunks {
            queue.push(Vec::from(chunk));
        }

        let mut futures = Vec::new();
        for _worker in 0..self.ctx.get_max_worker() {
            let ctx = self.ctx.clone();
            let queue = queue.clone();
            if !queue.is_empty() {
                futures.push(tokio::spawn(async move {
                    while !queue.is_empty() {
                        let range = queue.pop().await;
                        let (start, end) = (range[0], range[range.len() - 1]);
                        let range_path = format!("{}_{}", start, end);

                        let pipeline = Pipeline::create(&ctx, &range_path, range);
                        let res = pipeline.execute().await;
                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!(
                                    "Pipeline execute error will remove {:?}, error: {:?}",
                                    range_path,
                                    e
                                );
                            }
                        }
                    }
                }));
            }
        }

        for future in futures {
            future.await?;
        }
        Ok(())
    }
}
