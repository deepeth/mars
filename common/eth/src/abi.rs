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

// Transfer (index_topic_1 address from, index_topic_2 address to, uint256 value)
// Transfer (index_topic_1 address src, index_topic_2 address dst, uint256 wad)
// Transfer (index_topic_1 address from, index_topic_2 address to, index_topic_3 uint256 tokenId)
pub const ERC20_TOKEN_TRANSFER_SIG: &str =
    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

// https://etherscan.io/tx/0x0d779e56dad870e3dd074f0ad4d24614c47a725dbed0698c21028467e61c34b9#eventlog
// TransferSingle (index_topic_1 address operator, index_topic_2 address from, index_topic_3 address to, uint256 id, uint256 value)
pub const ERC1155_TRANSFER_SINGLE_SIG: &str =
    "0xc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62";

// https://etherscan.io/tx/0xfd818fa90e25092b6219fa7f7125f4a3bcade7d5bb302573da4bdb36c691ab1e#eventlog
// TransferBatch (index_topic_1 address operator, index_topic_2 address from, index_topic_3 address to, uint256[] ids, uint256[] values)
pub const ERC1155_TRANSFER_BATCH_SIG: &str =
    "0x4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb";

// https://etherscan.io/tx/0xc2ed0f5d895348382000056463b9b819b02b8d39cc784a137406b7311113ca24#eventlog
// NameRegistered (string name, index_topic_1 bytes32 label, index_topic_2 address owner, uint256 cost, uint256 expires)
pub const ENS_NAME_REGISTERED_SIG: &str =
    "0xca6abbe9d7f11422cb6ca7629fbf6fe9efb1c621f71ce8f02b9f2a230097404f";

// Other demos
// NFTs: https://etherscan.io/tx/0x1de541bae0b91097e39403f688a22eb6ee6eb6a226b8bf00f20851c4a9e7ac67
