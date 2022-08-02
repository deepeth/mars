CREATE TABLE IF NOT EXISTS nft_token_transfer(
    token_address STRING,
    from_address STRING,
    to_address STRING,
    token_id STRING,
    transaction_hash STRING,
    log_index BIGINT UNSIGNED,
    block_number BIGINT UNSIGNED,
);