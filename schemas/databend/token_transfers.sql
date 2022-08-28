CREATE TABLE token_transfers(
    token_address VARCHAR,
    from_address VARCHAR,
    to_address VARCHAR,
    token_id VARCHAR,
    value VARCHAR,
    erc_standard VARCHAR,
    transaction_hash VARCHAR,
    log_index BIGINT UNSIGNED,
    block_number BIGINT UNSIGNED
);