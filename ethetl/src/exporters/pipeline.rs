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

use common_exceptions::Result;

use crate::contexts::ContextRef;
use crate::exporters::BlockExporter;

pub struct Pipeline {
    ctx: ContextRef,
    block_numbers: Vec<usize>,
}

impl Pipeline {
    pub fn create(ctx: &ContextRef, block_numbers: Vec<usize>) -> Self {
        Self {
            ctx: ctx.clone(),
            block_numbers,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        // Create chunk dir.
        let start = self.block_numbers[0];
        let end = self.block_numbers[self.block_numbers.len() - 1];
        let dir = format!("{}/{}_{}", self.ctx.get_output_dir(), start, end);
        fs::create_dir_all(&dir)?;

        log::info!("Block[{} - {}] worker start", start, end);
        let export = BlockExporter::create(&self.ctx, &dir, self.block_numbers.to_vec());
        let res = export.export().await;
        log::info!("Block[{} - {}] worker end", start, end);
        res
    }
}
