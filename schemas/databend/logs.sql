CREATE TABLE logs (
    log_index BIGINT UNSIGNED,
    transaction_hash VARCHAR,
    transaction_index BIGINT UNSIGNED,
    block_hash VARCHAR,
    block_number BIGINT UNSIGNED,
    contract_address VARCHAR,
    data VARCHAR,
    topics VARCHAR
);