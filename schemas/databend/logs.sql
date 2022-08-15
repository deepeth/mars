CREATE TABLE logs (
    log_index BIGINT UNSIGNED,
    transaction_hash STRING,
    transaction_index BIGINT UNSIGNED,
    block_hash STRING,
    block_number BIGINT UNSIGNED,
    contract_address STRING,
    data STRING,
    topics STRING
);