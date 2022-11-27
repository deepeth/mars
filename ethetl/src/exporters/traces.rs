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

use arrow2::array::UInt64Array;
use arrow2::array::Utf8Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::DataType;
use arrow2::datatypes::Field;
use arrow2::datatypes::Schema;
use common_eth::bytes_to_hex;
use common_eth::h160_to_hex;
use common_eth::h256_to_hex;
use common_exceptions::Result;
use web3::types::Action;
use web3::types::Address;
use web3::types::Bytes;
use web3::types::Res;
use web3::types::Trace;
use web3::types::H256;
use web3::types::U256;

use crate::contexts::ContextRef;
use crate::exporters::write_file;

pub struct TracesExporter {
    ctx: ContextRef,
    output_dir: String,
    range_path: String,
}

impl TracesExporter {
    pub fn create(ctx: &ContextRef, output_dir: &str, range_path: &str) -> TracesExporter {
        Self {
            ctx: ctx.clone(),
            output_dir: output_dir.to_string(),
            range_path: range_path.to_string(),
        }
    }

    fn schema(&self) -> Schema {
        let block_number = Field::new("block_number", DataType::UInt64, true);
        let transaction_hash = Field::new("transaction_hash", DataType::Utf8, true);
        let transaction_index = Field::new("transaction_index", DataType::UInt64, true);
        let from_address = Field::new("from_address", DataType::Utf8, true);
        let to_address = Field::new("to_address", DataType::Utf8, true);
        let value = Field::new("value", DataType::UInt64, true);
        let input = Field::new("input", DataType::Utf8, true);
        let output = Field::new("output", DataType::Utf8, true);
        let trace_type = Field::new("trace_type", DataType::Utf8, true);
        let call_type = Field::new("call_type", DataType::Utf8, true);
        let reward_type = Field::new("reward_type", DataType::Utf8, true);
        let gas = Field::new("gas", DataType::UInt64, true);
        let gas_used = Field::new("gas_used", DataType::UInt64, true);
        let subtraces = Field::new("subtraces", DataType::UInt64, true);
        let trace_address = Field::new("trace_address", DataType::Utf8, true);
        let error = Field::new("error", DataType::Utf8, true);
        let status = Field::new("status", DataType::UInt64, true);
        let trace_id = Field::new("trace_id", DataType::Utf8, true);
        Schema::from(vec![
            block_number,
            transaction_hash,
            transaction_index,
            from_address,
            to_address,
            value,
            input,
            output,
            trace_type,
            call_type,
            reward_type,
            gas,
            gas_used,
            subtraces,
            trace_address,
            error,
            status,
            trace_id,
        ])
    }

    pub async fn export_traces(&self, traces: &[Trace]) -> Result<()> {
        let traces_len = traces.len();
        let mut block_number_vec = Vec::with_capacity(traces_len);
        let mut transaction_hash_vec = Vec::with_capacity(traces_len);
        let mut transaction_index_vec = Vec::with_capacity(traces_len);
        let mut from_address_vec = Vec::with_capacity(traces_len);
        let mut to_address_vec = Vec::with_capacity(traces_len);
        let mut value_vec = Vec::with_capacity(traces_len);
        let mut input_vec = Vec::with_capacity(traces_len);
        let mut output_vec = Vec::with_capacity(traces_len);
        let mut trace_type_vec = Vec::with_capacity(traces_len);
        let mut call_type_vec = Vec::with_capacity(traces_len);
        let mut reward_type_vec = Vec::with_capacity(traces_len);
        let mut gas_vec = Vec::with_capacity(traces_len);
        let mut gas_used_vec = Vec::with_capacity(traces_len);
        let mut subtraces_vec = Vec::with_capacity(traces_len);
        let mut trace_address_vec = Vec::with_capacity(traces_len);
        let mut error_vec = Vec::with_capacity(traces_len);
        let mut status_vec = Vec::with_capacity(traces_len);
        let mut trace_id_vec = Vec::with_capacity(traces_len);

        for trace in traces {
            block_number_vec.push(trace.block_number);
            transaction_hash_vec.push(h256_to_hex(
                &trace.transaction_hash.unwrap_or_else(H256::zero),
            ));
            transaction_index_vec.push(trace.transaction_position.unwrap_or(0) as u64);

            let res = trace.result.clone().unwrap_or_default();
            let (
                call_type,
                reward_type,
                from_address,
                to_address,
                input,
                output,
                value,
                gas,
                gas_used,
            ) = match &trace.action {
                Action::Call(v) => {
                    let gas_used = match res {
                        Res::Call(x) => x.gas_used,
                        _ => U256::zero(),
                    };
                    (
                        format!("{:?}", v.call_type),
                        "".to_string(),
                        v.from,
                        v.to,
                        v.input.clone(),
                        Bytes::default(),
                        v.value,
                        v.gas,
                        gas_used,
                    )
                }
                Action::Create(v) => {
                    let (to_address, gas_used, output) = match res {
                        Res::Create(x) => (x.address, x.gas_used, x.code),
                        _ => (Address::zero(), U256::zero(), Bytes::default()),
                    };
                    (
                        "".to_string(),
                        "".to_string(),
                        v.from,
                        to_address,
                        v.init.clone(),
                        output,
                        v.value,
                        v.gas,
                        gas_used,
                    )
                }
                Action::Suicide(v) => (
                    "".to_string(),
                    "".to_string(),
                    v.address,
                    v.refund_address,
                    Bytes::default(),
                    Bytes::default(),
                    v.balance,
                    U256::zero(),
                    U256::zero(),
                ),
                Action::Reward(v) => (
                    "".to_string(),
                    format!("{:?}", v.reward_type),
                    Address::zero(),
                    v.author,
                    Bytes::default(),
                    Bytes::default(),
                    v.value,
                    U256::zero(),
                    U256::zero(),
                ),
            };

            from_address_vec.push(h160_to_hex(&from_address));
            to_address_vec.push(h160_to_hex(&to_address));
            value_vec.push(value.as_u64());
            input_vec.push(bytes_to_hex(&input));
            output_vec.push(bytes_to_hex(&output));
            trace_type_vec.push(format!("{:?}", trace.action_type));
            call_type_vec.push(call_type);
            reward_type_vec.push(reward_type);
            gas_vec.push(gas.as_u64());
            gas_used_vec.push(gas_used.as_u64());
            subtraces_vec.push(trace.subtraces as u64);
            trace_address_vec.push(format!("{:?}", trace.trace_address));
            error_vec.push(trace.error.clone().unwrap_or_default());
            status_vec.push(0u64);
            trace_id_vec.push("");
        }

        let block_number_array = UInt64Array::from_slice(block_number_vec);
        let transaction_hash_array = Utf8Array::<i32>::from_slice(transaction_hash_vec);
        let transaction_index_array = UInt64Array::from_slice(transaction_index_vec);
        let from_address_array = Utf8Array::<i32>::from_slice(from_address_vec);
        let to_address_array = Utf8Array::<i32>::from_slice(to_address_vec);
        let value_array = UInt64Array::from_slice(value_vec);
        let input_array = Utf8Array::<i32>::from_slice(input_vec);
        let output_array = Utf8Array::<i32>::from_slice(output_vec);
        let trace_type_array = Utf8Array::<i32>::from_slice(trace_type_vec);
        let call_type_array = Utf8Array::<i32>::from_slice(call_type_vec);
        let reward_type_array = Utf8Array::<i32>::from_slice(reward_type_vec);
        let gas_array = UInt64Array::from_slice(gas_vec);
        let gas_used_array = UInt64Array::from_slice(gas_used_vec);
        let subtraces_array = UInt64Array::from_slice(subtraces_vec);
        let trace_address_array = Utf8Array::<i32>::from_slice(trace_address_vec);
        let error_array = Utf8Array::<i32>::from_slice(error_vec);
        let status_array = UInt64Array::from_slice(status_vec);
        let trace_id_array = Utf8Array::<i32>::from_slice(trace_id_vec);

        let columns = Chunk::try_new(vec![
            block_number_array.boxed(),
            transaction_hash_array.boxed(),
            transaction_index_array.boxed(),
            from_address_array.boxed(),
            to_address_array.boxed(),
            value_array.boxed(),
            input_array.boxed(),
            output_array.boxed(),
            trace_type_array.boxed(),
            call_type_array.boxed(),
            reward_type_array.boxed(),
            gas_array.boxed(),
            gas_used_array.boxed(),
            subtraces_array.boxed(),
            trace_address_array.boxed(),
            error_array.boxed(),
            status_array.boxed(),
            trace_id_array.boxed(),
        ])?;

        let path = format!("{}/traces/traces_{}", self.output_dir, self.range_path);
        write_file(&self.ctx, &path, self.schema(), columns, "traces").await
    }
}
