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

use opentelemetry::global;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

use crate::exceptions::Result;

pub fn init_tracing() -> Result<TracingStop> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let agent_addr = env::var("JAEGER_AGENT").unwrap_or_else(|_| "127.0.0.1:16686".to_string());

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("shafish")
        .with_agent_endpoint(agent_addr)
        .install_simple()?;

    let stderr_subscriber = tracing_subscriber::fmt::layer().pretty().with_target(true);

    let directives = env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_x| "info".to_string());
    let env_filter = EnvFilter::new(directives);

    let trace = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(trace)
        .with(stderr_subscriber);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(TracingStop { _priv: () })
}

pub struct TracingStop {
    _priv: (),
}

impl Drop for TracingStop {
    fn drop(&mut self) {
        global::shutdown_tracer_provider();
    }
}
