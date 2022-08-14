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
use web3::types::H160;
use web3::types::H2048;
use web3::types::H256;
use web3::types::H64;
use web3::types::U256;

pub fn h64_to_hex(v: &H64) -> String {
    let hex = hex::encode(v.0);
    "0x".to_owned() + hex.trim_start_matches('0')
}

pub fn h160_to_hex(v: &H160) -> String {
    let hex = hex::encode(v.0);
    "0x".to_owned() + hex.trim_start_matches('0')
}

// H256 to hex trim the start 0 and with the 0x prefix.
pub fn h256_to_hex(v: &H256) -> String {
    let hex = hex::encode(v.0);
    "0x".to_owned() + hex.trim_start_matches('0')
}

pub fn u256_to_hex(v: &U256) -> String {
    format!("{:#}", v)
}

pub fn h2048_to_hex(v: &H2048) -> String {
    let hex = hex::encode(v.0);
    "0x".to_owned() + hex.trim_start_matches('0')
}

// Bytes to hex with 0x prefix.
pub fn bytes_to_hex(v: &Bytes) -> String {
    "0x".to_string() + &v.0.iter().map(|x| format!("{:02x}", x)).collect::<String>()
}

// U256 type to f64 divide by 10000000, to make double work.
pub fn u256_to_f64(v: &U256) -> f64 {
    v.as_u128() as f64 / 10000000_f64
}
