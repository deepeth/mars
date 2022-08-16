CREATE TABLE transactions (
    hash STRING,
    nonce STRING,
    transaction_index BIGINT UNSIGNED,
    from_address STRING,
    to_address STRING,
    value DOUBLE,
    gas BIGINT UNSIGNED,
    gas_price BIGINT UNSIGNED,
    method_id STRING,
    input STRING,
    max_fee_per_gas BIGINT UNSIGNED,
    max_priority_fee_per_gas BIGINT UNSIGNED,
    transaction_type BIGINT UNSIGNED,
    block_hash STRING,
    block_number BIGINT UNSIGNED,
    block_timestamp BIGINT UNSIGNED
);
