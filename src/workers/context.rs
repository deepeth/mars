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

use crate::workers::Progress;
use crate::Config;

#[derive(Clone, Debug)]
pub struct Context {
    progress: Arc<Progress>,
    rpc_url: String,
    rpc_batch_size: usize,
    max_worker: usize,
}
pub type ContextRef = Arc<Context>;

impl Context {
    pub fn create(conf: &Config) -> Arc<Context> {
        let all = conf.end_block - conf.start_block + 1;
        Arc::new(Context {
            progress: Progress::create(all),
            rpc_url: conf.provider_uri.to_string(),
            rpc_batch_size: conf.batch_size,
            max_worker: conf.max_worker,
        })
    }

    pub fn get_rpc_url(&self) -> &str {
        &self.rpc_url
    }

    pub fn get_batch_size(&self) -> usize {
        self.rpc_batch_size
    }

    pub fn get_max_worker(&self) -> usize {
        self.max_worker
    }

    pub fn get_progress(&self) -> Arc<Progress> {
        self.progress.clone()
    }
}
