CREATE TABLE IF NOT EXISTS blocks
(
    number            BIGINT UNSIGNED,
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
    size              BIGINT UNSIGNED,
    extra_data        STRING,
    gas_limit         BIGINT UNSIGNED,
    gas_used          BIGINT UNSIGNED,
    timestamp         BIGINT UNSIGNED,
    transaction_count BIGINT UNSIGNED,
    base_fee_per_gas  BIGINT UNSIGNED
);