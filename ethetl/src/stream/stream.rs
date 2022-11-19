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


use crate::contexts::ContextRef;
use common_exceptions::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct StreamSyncing {
    start: u64,
    end :u64
}

pub struct  Stream {
  ctx: ContextRef,
}

impl Stream {
    pub fn create(ctx: ContextRef) -> Self {
        Stream{
            ctx,
        }
    }

    pub async fn start(&self)-> Result<()> {
        // Try to read stream_syncing.json
        let stream_syncing_file = "stream_syncing.json";
        let conf = self.ctx.get_config();
        let op = self.ctx.get_storage();

        if let Ok(syncing) = op.object(stream_syncing_file).read().await {

        } else {}
        todo!()
    }
}