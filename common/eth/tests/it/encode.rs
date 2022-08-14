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

use common_eth::*;
use web3::types::Bytes;
use web3::types::H160;
use web3::types::H2048;
use web3::types::H256;
use web3::types::H64;
use web3::types::U256;

#[test]
fn h64_to_hex_test() {
    let data = hex::decode("d4307b3ec19e6b6a").unwrap();
    let h64 = H64::from_slice(&data);
    let expect = "0xd4307b3ec19e6b6a";
    let actual = h64_to_hex(&h64);
    assert_eq!(expect, actual);
}

#[test]
fn h160_to_hex_test() {
    let data = hex::decode("cfef8857e9c80e3440a823971420f7fa5f62f020").unwrap();
    let h160 = H160::from_slice(&data);
    let expect = "0xcfef8857e9c80e3440a823971420f7fa5f62f020";
    let actual = h160_to_hex(&h160);
    assert_eq!(expect, actual);
}

#[test]
fn h256_to_hex_test() {
    {
        let h256 = H256::zero();
        let expect = "0x";
        let actual = h256_to_hex(&h256);
        assert_eq!(expect, actual);
    }

    {
        let data = hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
        let h256 = H256::from_slice(&data);
        let expect = "0x1";
        let actual = h256_to_hex(&h256);
        assert_eq!(expect, actual);
    }
}

#[test]
fn h2048_to_hex_test() {
    let bytes = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    let data = hex::decode(bytes).unwrap();
    let h2048 = H2048::from_slice(&data);
    let expect = "0x";
    let actual = h2048_to_hex(&h2048);
    assert_eq!(expect, actual);
}

#[test]
fn bytes_to_hex_test() {
    let data =
                hex::decode("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001")
                    .unwrap();
    let bytes = Bytes::from(data);

    let expect = "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001";
    let actual = bytes_to_hex(&bytes);
    assert_eq!(expect, actual);
}

#[test]
fn u256_to_f64_test() {
    let data =
        hex::decode("0000000000000000000000000000000000000000000000001326beb03e09ff95").unwrap();
    let u256 = U256::from_big_endian(&data);
    let expect = 1379999999999999893_f64 / 10000000_f64;
    let actual = u256_to_f64(&u256);
    assert_eq!(expect, actual);
}
