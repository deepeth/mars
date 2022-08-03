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

use std::io::Cursor;
use std::sync::Arc;

use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use arrow2::io::parquet::write::transverse;
use arrow2::io::parquet::write::CompressionOptions;
use arrow2::io::parquet::write::Encoding;
use arrow2::io::parquet::write::FileWriter;
use arrow2::io::parquet::write::RowGroupIterator;
use arrow2::io::parquet::write::Version;
use arrow2::io::parquet::write::WriteOptions;
use common_exceptions::Result;
use opendal::Operator;

pub async fn write_parquet(
    op: Arc<Operator>,
    path: &str,
    schema: Schema,
    columns: Chunk<Box<dyn Array>>,
) -> Result<()> {
    let options = WriteOptions {
        write_statistics: false,
        compression: CompressionOptions::Snappy,
        version: Version::V2,
    };

    let iter = vec![Ok(columns)];

    let encodings = schema
        .fields
        .iter()
        .map(|f| transverse(&f.data_type, |_| Encoding::Plain))
        .collect();
    let row_groups = RowGroupIterator::try_new(iter.into_iter(), &schema, options, encodings)?;

    let cursor = Cursor::new(Vec::new());
    let mut writer = FileWriter::try_new(cursor, schema, options)?;

    for group in row_groups {
        writer.write(group?)?;
    }
    let _size = writer.end(None)?;

    op.object(path)
        .write(writer.into_inner().get_ref().as_slice())
        .await?;
    Ok(())
}
