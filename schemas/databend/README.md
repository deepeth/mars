# Databend

Databend is an open-source Elastic and Workload-Aware modern cloud data warehouse, activate your object storage for real-time analytics. 

Github: https://github.com/datafuselabs/databend

Cloud: https://app.databend.com/

# Schema

## blocks.parquet

| Column            | Type            |
|-------------------|-----------------|
| number            | BIGINT UNSIGNED |
| hash              | VARCHAR         |
| parent_hash       | VARCHAR         |
| nonce             | VARCHAR         |
| sha3_uncles       | VARCHAR         |
| logs_bloom        | VARCHAR         |
| transactions_root | VARCHAR         |
| state_root        | VARCHAR         |
| receipts_root     | VARCHAR         |
| difficulty        | VARCHAR         |
| total_difficulty  | VARCHAR         |
| size              | BIGINT UNSIGNED |
| extra_data        | VARCHAR         |
| gas_limit         | BIGINT UNSIGNED |
| gas_used          | BIGINT UNSIGNED |
| timestamp         | BIGINT UNSIGNED |
| transaction_count | BIGINT UNSIGNED |
| base_fee_per_gas  | BIGINT UNSIGNED |


## transactions.parquet

| Column                   | Type            |
|--------------------------|-----------------|
| hash                     | VARCHAR         |
| nonce                    | VARCHAR         |
| transaction_index        | BIGINT UNSIGNED |
| from_address             | VARCHAR         |
| to_address               | VARCHAR         |
| value                    | DOUBLE          |
| gas                      | BIGINT UNSIGNED |
| method_id                | VARCHAR         |
| input                    | VARCHAR         |
| max_fee_per_gas          | BIGINT UNSIGNED |
| max_priority_fee_per_gas | BIGINT UNSIGNED |
| transaction_type         | BIGINT UNSIGNED |
| block_hash               | VARCHAR         |
| block_number             | BIGINT UNSIGNED |
| block_timestamp          | BIGINT UNSIGNED |

## logs.parquet

| Column            | Type            |
|-------------------|-----------------|
| log_index         | BIGINT UNSIGNED |
| transaction_hash  | VARCHAR         |
| transaction_index | BIGINT UNSIGNED |
| block_hash        | VARCHAR         |
| block_number      | BIGINT UNSIGNED |
| contract_address  | VARCHAR         |
| event_address     | VARCHAR         |
| data              | VARCHAR         |
| topics            | VARCHAR         |


## receipts.parquet

| Column               | Type              |
|----------------------|-------------------|
| transaction_hash     | VARCHAR           |
| transaction_index    | BIGINT UNSIGNED   |
| block_hash           | VARCHAR           |
| block_number         | BIGINT UNSIGNED   |
| cumulative_gas_used  | BIGINT UNSIGNED   |
| gas_used             | BIGINT UNSIGNED   |
| contract_address     | VARCHAR           |
| root                 | VARCHAR           |
| status               | BIGINT UNSIGNED   |
| effective_gas_price  | BIGINT UNSIGNED   |

## token_transfers.parquet

| Column             | Type              |
|--------------------|-------------------|
| token_address      | VARCHAR           |
| from_address       | VARCHAR           |
| to_address         | VARCHAR           |
| token_id           | VARCHAR           |
| value              | VARCHAR           |
| erc_standard       | VARCHAR           |
| transaction_hash   | VARCHAR           |
| log_index          | BIGINT UNSIGNED   |
| block_number       | BIGINT UNSIGNED   |

## ens.parquet

| Column             | Type            |
|--------------------|-----------------|
| name               | VARCHAR         |
| cost               | DOUBLE          |
| expires            | BIGINT UNSIGNED |
| owner              | VARCHAR         |
| transaction_hash   | VARCHAR         |
| block_number       | BIGINT UNSIGNED |
