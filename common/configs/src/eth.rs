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
// Copy from https://github.com/Sherlock-Holo/ddns/blob/master/src/trace.rs

use std::env;

use clap::Parser;
use common_exceptions::Result;
use serde::Deserialize;
use serde::Serialize;
use serfig::collectors::from_env;
use serfig::collectors::from_file;
use serfig::collectors::from_self;
use serfig::parsers::Toml;

use crate::LogConfig;
use crate::StorageConfig;

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ExportConfig {
    #[clap(
        short = 'p',
        long,
        env,
        value_parser,
        default_value_t,
        help = "The URI of the web3 provider e.g. https://127.0.0.1:8845"
    )]
    pub provider_uri: String,

    #[clap(
        short = 's',
        long,
        value_parser,
        default_value_t = 0,
        help = "Start block"
    )]
    pub start_block: usize,

    #[clap(
        short = 'e',
        long,
        value_parser,
        default_value_t = 10000,
        help = "End block"
    )]
    pub end_block: usize,

    #[clap(
        short = 'b',
        long,
        value_parser,
        default_value_t = 1000,
        help = "The number of items to export at a time"
    )]
    pub batch_size: usize,

    #[clap(
        short = 'w',
        long,
        value_parser,
        default_value_t = 8,
        help = "The maximum number of workers"
    )]
    pub max_worker: usize,

    #[clap(long, value_parser, default_value_t = 100)]
    pub web3_batch_size: usize,

    #[clap(
        long,
        value_parser,
        default_value_t = 60,
        help = "Syncing with eth node every N seconds"
    )]
    pub syncing_interval_secs: usize,

    #[clap(
        short = 'o',
        long,
        value_parser,
        default_value = "_datas",
        help = "Exporter directory"
    )]
    pub output_dir: String,
}

impl Default for ExportConfig {
    fn default() -> Self {
        ExportConfig {
            provider_uri: "https://127.0.0.1:8545".to_string(),
            start_block: 0,
            end_block: 10000,
            batch_size: 10000,
            max_worker: 4,
            web3_batch_size: 1000,
            syncing_interval_secs: 2,
            output_dir: "_datas".to_string(),
        }
    }
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EthConfig {
    #[clap(flatten)]
    pub log: LogConfig,

    #[clap(flatten)]
    pub export: ExportConfig,

    #[clap(flatten)]
    pub storage: StorageConfig,

    #[clap(long, short = 'c', default_value_t)]
    pub config_file: String,
}

impl Default for EthConfig {
    fn default() -> Self {
        EthConfig {
            log: Default::default(),
            export: Default::default(),
            storage: Default::default(),
            config_file: "".to_string(),
        }
    }
}

impl EthConfig {
    /// Load will load config from file, env and args.
    ///
    /// - Load from file as default.
    /// - Load from env, will override config from file.
    /// - Load from args as finally override
    pub fn load() -> Result<Self> {
        let arg_conf = Self::parse();
        let mut builder: serfig::Builder<Self> = serfig::Builder::default();

        // Load from config file first.
        {
            let config_file = if !arg_conf.config_file.is_empty() {
                arg_conf.config_file.clone()
            } else if let Ok(path) = env::var("CONFIG_FILE") {
                path
            } else {
                "".to_string()
            };

            builder = builder.collect(from_file(Toml, &config_file));
        }

        // Then, load from env.
        builder = builder.collect(from_env());

        // Finally, load from args.
        builder = builder.collect(from_self(arg_conf));
        builder.build()
    }
}
