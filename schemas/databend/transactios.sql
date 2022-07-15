CREATE TABLE IF NOT EXISTS transactions (
    hash STRING,
    nonce STRING,
    transaction_index BIGINT,
    from_address STRING,
    to_address STRING,
    value STRING,
    gas BIGINT,
    gas_price BIGINT,
    input STRING,
    max_fee_per_gas BIGINT,
    max_priority_fee_per_gas BIGINT,
    transaction_type BIGINT,
    block_hash STRING,
    block_number BIGINT,
    block_timestamp BIGINT
);