CREATE TABLE receipts (
    transaction_hash STRING,
    transaction_index BIGINT UNSIGNED,
    block_hash STRING,
    block_number BIGINT UNSIGNED,
    cumulative_gas_used BIGINT UNSIGNED,
    gas_used BIGINT UNSIGNED,
    contract_address STRING,
    root STRING,
    status BIGINT UNSIGNED,
    effective_gas_price BIGINT UNSIGNED
);