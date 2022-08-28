CREATE TABLE receipts (
    transaction_hash VARCHAR,
    transaction_index BIGINT UNSIGNED,
    block_hash VARCHAR,
    block_number BIGINT UNSIGNED,
    cumulative_gas_used BIGINT UNSIGNED,
    gas_used BIGINT UNSIGNED,
    contract_address VARCHAR,
    root VARCHAR,
    status BIGINT UNSIGNED,
    effective_gas_price BIGINT UNSIGNED
);