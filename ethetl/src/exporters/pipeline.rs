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

use common_exceptions::Result;

use crate::contexts::ContextRef;
use crate::exporters::BlockExporter;

pub struct Pipeline {
    ctx: ContextRef,
    block_numbers: Vec<usize>,
    output_dir: String,
    range_path: String,
}

impl Pipeline {
    pub fn create(ctx: &ContextRef, range_path: &str, block_numbers: Vec<usize>) -> Self {
        Self {
            ctx: ctx.clone(),
            output_dir: ctx.get_output_dir().to_string(),
            range_path: range_path.to_string(),
            block_numbers,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let export = BlockExporter::create(
            &self.ctx,
            &self.output_dir,
            &self.range_path,
            self.block_numbers.to_vec(),
        );
        let res = export.export().await;
        res
    }
}
