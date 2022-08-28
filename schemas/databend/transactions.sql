CREATE TABLE transactions (
    hash VARCHAR,
    nonce VARCHAR,
    transaction_index BIGINT UNSIGNED,
    from_address VARCHAR,
    to_address VARCHAR,
    value DOUBLE,
    gas BIGINT UNSIGNED,
    gas_price BIGINT UNSIGNED,
    method_id VARCHAR,
    input VARCHAR,
    max_fee_per_gas BIGINT UNSIGNED,
    max_priority_fee_per_gas BIGINT UNSIGNED,
    transaction_type BIGINT UNSIGNED,
    block_hash VARCHAR,
    block_number BIGINT UNSIGNED,
    block_timestamp BIGINT UNSIGNED
);
