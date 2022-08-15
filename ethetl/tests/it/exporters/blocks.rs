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
use std::path::Path;

use common_exceptions::Result;
use ethetl::exporters::BlockExporter;

use crate::common::create_config;
use crate::common::create_ctx;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_blocks_exporters() -> Result<()> {
    let mut conf = create_config();
    conf.export.start_block = 15340159;
    conf.export.end_block = 15340160;
    let ctx = create_ctx(&conf).await;

    let dir = format!(
        "{}/{}_{}",
        ctx.get_output_dir(),
        conf.export.start_block,
        conf.export.end_block
    );
    fs::create_dir_all(&dir)?;

    let range: Vec<usize> = (conf.export.start_block..conf.export.end_block + 1).collect();

    // CSV.
    {
        let exporter = BlockExporter::create(&ctx, &dir, range.to_vec());
        exporter.export().await?;

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15340159_15340160/blocks.csv"),
            Path::new("_datas/_test_output_dir/15340159_15340160/blocks.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15340159_15340160/transactions.csv"),
            Path::new("_datas/_test_output_dir/15340159_15340160/transactions.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15340159_15340160/_transaction_hashes.txt"),
            Path::new("_datas/_test_output_dir/15340159_15340160/_transaction_hashes.txt"),
        );
    }

    // Parquet.
    {
        /*
        conf.export.output_format = "parquet".to_string();
        let exporter = BlockExporter::create(&ctx, &dir, range);
        exporter.export().await?;

        goldenfile::differs::binary_diff(
            Path::new("tests/it/testdata/15340159_15340160/blocks.parquet"),
            Path::new("_datas/_test_output_dir/15340159_15340160/blocks.parquet"),
        );

        goldenfile::differs::binary_diff(
            Path::new("tests/it/testdata/15340159_15340160/transactions.parquet"),
            Path::new("_datas/_test_output_dir/15340159_15340160/transactions.parquet"),
        );

         */
    }

    Ok(())
}
