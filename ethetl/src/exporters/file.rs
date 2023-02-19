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

use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use common_exceptions::Error;
use common_exceptions::Result;

use crate::contexts::ContextRef;

pub async fn write_file(
    ctx: &ContextRef,
    path: &str,
    schema: Schema,
    columns: Chunk<Box<dyn Array>>,
    msg: &str,
) -> Result<()> {
    debug_assert!(
        schema
            .fields
            .iter()
            .zip(columns.iter())
            .all(|(dt1, dt2)| &dt1.data_type == dt2.data_type()),
        "schema={schema:?}\ncolumns={:?}",
        columns.iter().map(|v| v.data_type()).collect::<Vec<_>>()
    );

    match ctx.get_output_format().to_lowercase().as_str() {
        "csv" => {
            let path = format!("{}.csv", path);
            log::info!("Write {} to {}", msg, path);
            common_storages::write_csv(ctx.get_storage(), &path, schema, columns).await
        }
        "parquet" => {
            let path = format!("{}.parquet", path);
            log::info!("Write {} to {}", msg, path);
            common_storages::write_parquet(ctx.get_storage(), &path, schema, columns).await
        }
        v => Err(Error::msg(format!(
            "Unsupported format, must be one of [csv, parquet], got: {}",
            v
        ))),
    }
}
