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
#[ignore]
async fn test_blocks_exporters() -> Result<()> {
    let mut conf = create_config();
    conf.start_block = 15138828;
    conf.end_block = 15138852;
    let ctx = create_ctx(&conf);

    let dir = format!(
        "{}/{}_{}",
        ctx.get_output_dir(),
        conf.start_block,
        conf.end_block
    );
    fs::create_dir_all(&dir)?;

    let range: Vec<usize> = (conf.start_block..conf.end_block + 1).collect();

    // CSV.
    {
        let exporter = BlockExporter::create(&ctx, &dir, range.to_vec());
        exporter.export().await?;

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15138828_15138852/blocks.csv"),
            Path::new("_test_output_dir/15138828_15138852/blocks.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15138828_15138852/transactions.csv"),
            Path::new("_test_output_dir/15138828_15138852/transactions.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15138828_15138852/.transaction_hashes.txt"),
            Path::new("_test_output_dir/15138828_15138852/.transaction_hashes.txt"),
        );
    }

    // Parquet.
    {
        conf.output_format = "parquet".to_string();
        let exporter = BlockExporter::create(&ctx, &dir, range);
        exporter.export().await?;

        goldenfile::differs::binary_diff(
            Path::new("tests/it/testdata/15138828_15138852/blocks.parquet"),
            Path::new("_test_output_dir/15138828_15138852/blocks.parquet"),
        );

        goldenfile::differs::binary_diff(
            Path::new("tests/it/testdata/15138828_15138852/transactions.parquet"),
            Path::new("_test_output_dir/15138828_15138852/transactions.parquet"),
        );
    }

    Ok(())
}
