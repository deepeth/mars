CREATE TABLE IF NOT EXISTS receipts (
    transaction_hash STRING,
    transaction_index BIGINT UNSIGNED,
    block_hash STRING,
    block_number BIGINT UNSIGNED,
    cumulative_gas_used BIGINT UNSIGNED,
    gas_used BIGINT UNSIGNED,
    contract_address STRING,
    status BIGINT UNSIGNED,
    root STRING,
    effective_gas_price BIGINT UNSIGNED
);