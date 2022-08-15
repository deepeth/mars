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
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use common_exceptions::Result;
use ethetl::exporters::ReceiptExporter;
use web3::types::H256;

use crate::common::create_config;
use crate::common::create_ctx;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_receipts_exporters() -> Result<()> {
    let mut conf = create_config();
    conf.export.start_block = 15340159;
    conf.export.end_block = 15340160;
    let ctx = create_ctx(&conf).await;

    let path = "tests/it/testdata/15340159_15340160/_transaction_hashes.txt";
    let file = File::open(path)?;
    let buffered = BufReader::new(file);

    let mut tx_hashes = vec![];
    for line in buffered.lines() {
        let line_str = &line?;
        tx_hashes.push(H256::from_str(line_str).unwrap());
    }

    let dir = format!(
        "{}/{}_{}",
        ctx.get_output_dir(),
        conf.export.start_block,
        conf.export.end_block
    );
    fs::create_dir_all(&dir)?;

    // CSV.
    {
        let exporter = ReceiptExporter::create(&ctx, &dir, tx_hashes.to_vec());
        exporter.export().await?;

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15340159_15340160/receipts.csv"),
            Path::new("_datas/_test_output_dir/15340159_15340160/receipts.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15340159_15340160/logs.csv"),
            Path::new("_datas/_test_output_dir/15340159_15340160/logs.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15340159_15340160/token_transfers.csv"),
            Path::new("_datas/_test_output_dir/15340159_15340160/token_transfers.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/15340159_15340160/ens.csv"),
            Path::new("_datas/_test_output_dir/15340159_15340160/ens.csv"),
        );
    }

    // Parquet
    {
        /*
            conf.export.output_format = "parquet".to_string();
            let exporter = ReceiptExporter::create(&ctx, &dir, tx_hashes);
            exporter.export().await?;

            goldenfile::differs::binary_diff(
                Path::new("tests/it/testdata/15340159_15340160/receipts.parquet"),
                Path::new("_datas/_test_output_dir/15340159_15340160/receipts.parquet"),
            );
        */
    }
    Ok(())
}
