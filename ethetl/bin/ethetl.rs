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
use common_eth::Eth;
use common_exceptions::Result;
use env_logger::Builder;
use env_logger::Env;
use ethetl::contexts::Context;
use ethetl::exporters::Worker;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).format_target(false).init();

    let conf = EthConfig::load()?;
    log::info!("Config: {:?}", conf);

    let ctx = Context::create(&conf).await;

    let eth = Eth::create(&conf.export.provider_uri);
    let blk = eth.block_number().await?;
    log::info!("Eth node latest number:{:}", blk);

    // Interval progress.
    let progress = ctx.get_progress();
    progress.start();

    // Exporter.
    let start = conf.export.start_block;
    let end = conf.export.end_block;
    let range: Vec<usize> = (start..end + 1).collect();

    // Worker.
    let worker = Worker::create(&ctx, range);
    worker.start().await?;

    Ok(())
}
