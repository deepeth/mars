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
use common_exceptions::Result;
use env_logger::Builder;
use env_logger::Env;
use ethetl::contexts::Context;
use ethetl::etl::NormalEtl;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).format_target(false).init();

    let conf = EthConfig::load()?;
    log::info!("Config: {:?}", conf);

    // Create data dir.
    let ctx = Context::create(&conf).await;

    // Interval progress.
    let progress = ctx.get_progress();
    progress.set_name("Normal".to_string());
    progress.start();

    let normal = NormalEtl::create(ctx);
    normal.start().await?;
    progress.stop();

    Ok(())
}
