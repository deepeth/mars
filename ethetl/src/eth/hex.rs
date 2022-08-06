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

use web3::types::Bytes;
use web3::types::H256;

pub fn h256_to_hex(v: &H256) -> String {
    let hex = v
        .as_bytes()
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    hex.trim_start_matches('0').to_string()
}

pub fn bytes_to_hex(v: &Bytes) -> String {
    v.0.iter().map(|x| format!("{:02x}", x)).collect::<String>()
}
