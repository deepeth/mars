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
use web3::types::Bytes;
use web3::types::U256;

pub fn decode_with_types(types: &[String], data: &str) -> anyhow::Result<Vec<Token>> {
    let types: Vec<ParamType> = types
        .iter()
        .map(|s| Reader::read(s))
        .collect::<Result<_, _>>()?;

    let data: Vec<u8> = hex::decode(data)?;
    let tokens = decode(&types, &data)?;
    assert_eq!(types.len(), tokens.len());
    Ok(tokens)
}

// Transfer (index_topic_1 address from, index_topic_2 address to, uint256 value)
pub fn decode_u256_data(bytes: &Bytes) -> anyhow::Result<U256> {
    let bytes = hex::encode(&bytes.0);
    let types = vec!["uint".to_string()];
    let tokens = decode_with_types(&types, &bytes)?;
    match tokens[0] {
        Token::Uint(x) => Ok(x),
        _ => Ok(U256::zero()),
    }
}

// TransferSingle (index_topic_1 address operator, index_topic_2 address from, index_topic_3 address to, uint256 id, uint256 value)
pub fn decode_transfer_single_data(bytes: &Bytes) -> anyhow::Result<Option<(U256, U256)>> {
    let bytes = hex::encode(&bytes.0);
    let types = vec!["uint".to_string(), "uint".to_string()];
    let tokens = decode_with_types(&types, &bytes)?;
    if tokens.len() < 2 {
        return Ok(None);
    }

    match (&tokens[0], &tokens[1]) {
        (Token::Uint(a), Token::Uint(b)) => Ok(Some((*a, *b))),
        _ => Ok(None),
    }
}

// TransferBatch (index_topic_1 address operator, index_topic_2 address from, index_topic_3 address to, uint256[] ids, uint256[] values)
pub fn decode_transfer_batch_data(bytes: &Bytes) -> anyhow::Result<Option<(Vec<U256>, Vec<U256>)>> {
    let bytes = hex::encode(&bytes.0);
    let types = vec!["uint[]".to_string(), "uint[]".to_string()];
    let tokens = decode_with_types(&types, &bytes)?;
    if tokens.len() < 2 {
        return Ok(None);
    }

    match (&tokens[0], &tokens[1]) {
        (Token::Array(a), Token::Array(b)) => {
            let a_uint_vec = a
                .iter()
                .map(|x| match x {
                    Token::Uint(v) => *v,
                    _ => U256::zero(),
                })
                .collect::<Vec<U256>>();

            let b_uint_vec = b
                .iter()
                .map(|x| match x {
                    Token::Uint(v) => *v,
                    _ => U256::zero(),
                })
                .collect::<Vec<U256>>();
            Ok(Some((a_uint_vec, b_uint_vec)))
        }
        _ => Ok(None),
    }
}

// NameRegistered (string name, index_topic_1 bytes32 label, index_topic_2 address owner, uint256 cost, uint256 expires)
pub fn decode_name_registered_data(bytes: &Bytes) -> anyhow::Result<Option<(String, U256, U256)>> {
    let bytes = hex::encode(&bytes.0);
    let types = vec!["string".to_string(), "uint".to_string(), "uint".to_string()];
    let tokens = decode_with_types(&types, &bytes)?;
    if tokens.len() < 3 {
        return Ok(None);
    }

    match (&tokens[0], &tokens[1], &tokens[2]) {
        (Token::String(a), Token::Uint(b), Token::Uint(c)) => Ok(Some((a.clone(), *b, *c))),
        _ => Ok(None),
    }
}
