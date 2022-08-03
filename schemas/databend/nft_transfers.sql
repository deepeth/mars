CREATE TABLE IF NOT EXISTS nft_transfers(
    token_address STRING,
    from_address STRING,
    to_address STRING,
    token_id STRING,
    erc_standard STRING,
    transaction_hash STRING,
    log_index BIGINT UNSIGNED,
    block_number BIGINT UNSIGNED
);