// Copyright 2022 Datafuse Labs.
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

use std::backtrace::Backtrace;
use std::sync::Arc;

use crate::ErrorCode;
use crate::ErrorCodeBacktrace;

impl From<web3::Error> for ErrorCode {
    fn from(error: web3::Error) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}

impl From<opentelemetry::trace::TraceError> for ErrorCode {
    fn from(error: opentelemetry::trace::TraceError) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}

impl From<tracing::dispatcher::SetGlobalDefaultError> for ErrorCode {
    fn from(error: tracing::dispatcher::SetGlobalDefaultError) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}

impl From<serde_json::Error> for ErrorCode {
    fn from(error: serde_json::Error) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}

impl From<std::io::Error> for ErrorCode {
    fn from(error: std::io::Error) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}

impl From<arrow2::error::Error> for ErrorCode {
    fn from(error: arrow2::error::Error) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}

impl From<tokio::task::JoinError> for ErrorCode {
    fn from(error: tokio::task::JoinError) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}

impl From<anyhow::Error> for ErrorCode {
    fn from(error: anyhow::Error) -> Self {
        ErrorCode::create(
            1002,
            format!("{:?}", error),
            None,
            Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture()))),
        )
    }
}
