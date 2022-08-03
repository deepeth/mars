CREATE TABLE IF NOT EXISTS logs (
    log_index BIGINT UNSIGNED,
    transaction_hash STRING,
    transaction_index BIGINT UNSIGNED,
    block_hash STRING,
    block_number STRING,
    contract_adderss STRING,
    data STRING,
    topics STRING
);