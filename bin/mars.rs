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

use env_logger::Env;
use mars::configs::Config;
use mars::contexts::Context;
use mars::exporters::BlockExporter;
use mars::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let conf = Config::load()?;
    log::info!("Config: {:?}", conf);

    let ctx = Context::create(&conf);

    // Interval progress.
    let progress = ctx.get_progress();
    progress.start();

    // Exporter.
    let start = conf.start_block;
    let end = conf.end_block;
    let range: Vec<usize> = (start..end + 1).collect();
    let block_exporter = BlockExporter::create(&ctx, range);
    block_exporter.export().await?;

    Ok(())
}
