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

use common_configs::EthConfig;
use common_configs::ExportConfig;
use ethetl::contexts::Context;
use ethetl::contexts::ContextRef;

pub fn create_config() -> EthConfig {
    // export PROVIDER_URI='<your-provider-uri>'
    let provider_uri = std::env::var("PROVIDER_URI").unwrap();

    EthConfig {
        export: ExportConfig {
            provider_uri,
            start_block: 16600001,
            end_block: 16600002,
            batch_size: 100,
            max_worker: 4,
            web3_batch_size: 50,
            syncing_interval_secs: 1,
            output_dir: "_test_output_dir".to_string(),
        },
        ..Default::default()
    }
}

pub async fn create_ctx(conf: &EthConfig) -> ContextRef {
    Context::create(conf).await
}
