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

use std::fs;

use clap::Parser;
use common_exceptions::Result;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    #[clap(
        short = 'p',
        long,
        value_parser,
        help = "The URI of the web3 provider e.g. https://127.0.0.1:8845"
    )]
    pub provider_uri: String,

    #[clap(short = 's', long, value_parser, help = "Start block")]
    pub start_block: usize,

    #[clap(short = 'e', long, value_parser, help = "End block")]
    pub end_block: usize,

    #[clap(
        short = 'b',
        long,
        value_parser,
        default_value_t = 10000,
        help = "The number of items to export at a time"
    )]
    pub batch_size: usize,

    #[clap(
        short = 'w',
        long,
        value_parser,
        default_value_t = 4,
        help = "The maximum number of workers"
    )]
    pub max_worker: usize,

    #[clap(
        short = 'o',
        long,
        value_parser,
        default_value = ".datas",
        help = "Exporter directory"
    )]
    pub output_dir: String,

    #[clap(
        short = 'f',
        long,
        value_parser,
        default_value = "csv",
        help = "Exporter Format(csv|parquet)"
    )]
    pub output_format: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut conf = Self::parse();

        let start = conf.start_block;
        let end = conf.end_block;
        if conf.output_dir.is_empty() {
            conf.output_dir = format!("datas/{}_{}", start, end);
        }
        fs::create_dir_all(&conf.output_dir)?;

        Ok(conf)
    }
}
