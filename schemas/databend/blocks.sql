CREATE TABLE blocks
(
    number            BIGINT UNSIGNED,
    hash              VARCHAR,
    parent_hash       VARCHAR,
    nonce             VARCHAR,
    sha3_uncles       VARCHAR,
    logs_bloom        VARCHAR,
    transactions_root VARCHAR,
    state_root        VARCHAR,
    receipts_root     VARCHAR,
    difficulty        VARCHAR,
    total_difficulty  VARCHAR,
    size              BIGINT UNSIGNED,
    extra_data        VARCHAR,
    gas_limit         BIGINT UNSIGNED,
    gas_used          BIGINT UNSIGNED,
    timestamp         BIGINT UNSIGNED,
    transaction_count BIGINT UNSIGNED,
    base_fee_per_gas  BIGINT UNSIGNED
);