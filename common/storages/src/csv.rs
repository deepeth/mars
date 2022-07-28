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

use std::io::Cursor;
use std::sync::Arc;

use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use arrow2::io::csv::write;
use common_exceptions::Result;
use opendal::Operator;

pub async fn write_csv(
    op: Arc<Operator>,
    path: &str,
    schema: Schema,
    columns: Chunk<Box<dyn Array>>,
) -> Result<()> {
    let mut c = Cursor::new(Vec::new());
    let headers = schema
        .fields
        .iter()
        .map(|f| f.name.clone())
        .collect::<Vec<String>>();

    let options = write::SerializeOptions::default();
    write::write_header(&mut c, headers.as_slice(), &options)?;
    write::write_chunk(&mut c, &columns, &options)?;

    op.object(path).write(c.get_ref().as_slice()).await?;
    Ok(())
}
