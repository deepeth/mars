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

use common_configs::EthConfig;
use common_storages::init_fs_storage;
use opendal::Operator;

use crate::contexts::Progress;

#[derive(Clone, Debug)]
pub struct Context {
    progress: Arc<Progress>,
    rpc_url: String,
    batch_size: usize,
    max_worker: usize,
    web3_batch_size: usize,
    output_dir: String,
    output_format: String,
    storage: Arc<Operator>,
}
pub type ContextRef = Arc<Context>;

impl Context {
    pub async fn create(conf: &EthConfig) -> Arc<Context> {
        let all = conf.export.end_block - conf.export.start_block + 1;
        let storage = Arc::new(init_fs_storage(&conf.export.output_dir).await.unwrap());

        Arc::new(Context {
            progress: Progress::create(all),
            rpc_url: conf.export.provider_uri.to_string(),
            batch_size: conf.export.batch_size,
            max_worker: conf.export.max_worker,
            web3_batch_size: conf.export.web3_batch_size,
            output_dir: conf.export.output_dir.clone(),
            output_format: conf.export.output_format.clone(),
            storage,
        })
    }

    pub fn get_rpc_url(&self) -> &str {
        &self.rpc_url
    }

    pub fn get_batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn get_max_worker(&self) -> usize {
        self.max_worker
    }

    pub fn get_web3_batch_size(&self) -> usize {
        self.web3_batch_size
    }

    pub fn get_progress(&self) -> Arc<Progress> {
        self.progress.clone()
    }

    pub fn get_output_dir(&self) -> &str {
        &self.output_dir
    }

    pub fn get_storage(&self) -> Arc<Operator> {
        self.storage.clone()
    }

    pub fn get_output_format(&self) -> &str {
        &self.output_format
    }
}
