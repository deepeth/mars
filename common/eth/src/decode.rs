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

use web3::ethabi::decode;
use web3::ethabi::param_type::Reader;
use web3::ethabi::ParamType;
use web3::ethabi::Token;
use web3::ethabi::Uint;
use web3::types::Bytes;

pub fn bytes_to_u256(bytes: Bytes) -> anyhow::Result<Uint> {
    let bytes = bytes
        .0
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    let types = vec!["uint".to_string()];
    let tokens = decode_with_types(&types, &bytes)?;
    match tokens[0] {
        Token::Uint(x) => Ok(x),
        _ => Ok(Uint::zero()),
    }
}

pub fn decode_with_types(types: &[String], data: &str) -> anyhow::Result<Vec<Token>> {
    let types: Vec<ParamType> = types
        .iter()
        .map(|s| Reader::read(s))
        .collect::<Result<_, _>>()?;

    let data: Vec<u8> = hex::decode(&data)?;
    let tokens = decode(&types, &data)?;
    assert_eq!(types.len(), tokens.len());
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bytes_to_u256_test() {
        // https://cn.etherscan.com/tx/0x40fda4bee11e97340cb3cb29dfc479f07a516ee946c5d6f7e41d1c3a99b87c57#eventlog
        {
            let types = vec!["uint".to_string()];
            let data = "000000000000000000000000000000000000000000000017112108b7e7f1ba68";
            let expect = "[Uint(425509391054159329896)]";
            let actual = format!("{:?}", decode_with_types(&types, data).unwrap());
            assert_eq!(expect, actual);
        }

        {
            let expect = "425509391054159329896";
            let data =
                hex::decode("000000000000000000000000000000000000000000000017112108b7e7f1ba68")
                    .unwrap();
            let bytes = Bytes::from(data);
            let actual = format!("{:?}", bytes_to_u256(bytes).unwrap());
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn decode_normal_types_test() {
        // https://etherscan.io/tx/0xc2ed0f5d895348382000056463b9b819b02b8d39cc784a137406b7311113ca24#eventlog
        let types = vec!["string".to_string(), "uint".to_string(), "uint".to_string()];
        let data = "000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000033afeca7f3dc500000000000000000000000000000000000000000000000000000000638714c800000000000000000000000000000000000000000000000000000000000000083030303030323334000000000000000000000000000000000000000000000000";
        let expect = "[String(\"00000234\"), Uint(909290923572677), Uint(1669797064)]";
        let actual = format!("{:?}", decode_with_types(&types, data).unwrap());
        assert_eq!(expect, actual);
    }

    #[test]
    fn decode_array_types_test() {
        // https://cn.etherscan.com/tx/0xfd818fa90e25092b6219fa7f7125f4a3bcade7d5bb302573da4bdb36c691ab1e#eventlog
        let types = vec!["uint[]".to_string(), "uint[]".to_string()];
        let data = "000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000027a9fe22691c811ea339d9b73150e6911a5343dca0000000000000000060090007a9fe22691c811ea339d9b73150e6911a5343dca000000000000000006009001000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001";
        let expect = "[Array([Uint(55464657044963196816950587289035428064568320970692304673817341489687488925696), Uint(55464657044963196816950587289035428064568320970692304673817341489687488925697)]), Array([Uint(1), Uint(1)])]";
        let actual = format!("{:?}", decode_with_types(&types, data).unwrap());
        assert_eq!(expect, actual);
    }
}
