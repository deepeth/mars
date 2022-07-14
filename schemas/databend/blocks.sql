CREATE TABLE IF NOT EXISTS blocks
(
    number            BIGINT,
    hash              STRING,
    parent_hash       STRING,
    nonce             STRING,
    sha3_uncles       STRING,
    logs_bloom        STRING,
    transactions_root STRING,
    state_root        STRING,
    receipts_root     STRING,
    difficulty        STRING,
    total_difficulty  STRING,
    size              BIGINT,
    extra_data        STRING,
    gas_limit         BIGINT,
    gas_used          BIGINT,
    timestamp         BIGINT,
    transaction_count BIGINT,
    base_fee_per_gas  BIGINT
);