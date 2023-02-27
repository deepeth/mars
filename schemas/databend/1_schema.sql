DROP DATABASE IF EXISTS eth;

CREATE DATABASE eth;

USE eth;

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
    timestamp         TIMESTAMP,
    transaction_count BIGINT UNSIGNED,
    base_fee_per_gas  BIGINT UNSIGNED
);

CREATE TABLE transactions
(
    hash                     VARCHAR,
    nonce                    VARCHAR,
    transaction_index        BIGINT UNSIGNED,
    from_address             VARCHAR,
    to_address               VARCHAR,
    value                    DECIMAL(36, 18),
    gas                      BIGINT UNSIGNED,
    gas_price                BIGINT UNSIGNED,
    method_id                VARCHAR,
    input                    VARCHAR,
    max_fee_per_gas          BIGINT UNSIGNED,
    max_priority_fee_per_gas BIGINT UNSIGNED,
    transaction_type         BIGINT UNSIGNED,
    block_hash               VARCHAR,
    block_number             BIGINT UNSIGNED,
    block_timestamp          TIMESTAMP
);

CREATE TABLE logs
(
    log_index         BIGINT UNSIGNED,
    transaction_hash  VARCHAR,
    transaction_index BIGINT UNSIGNED,
    block_hash        VARCHAR,
    block_number      BIGINT UNSIGNED,
    contract_address  VARCHAR,
    event_address     VARCHAR,
    data              VARCHAR,
    topics            VARCHAR
);

CREATE TABLE receipts
(
    transaction_hash    VARCHAR,
    transaction_index   BIGINT UNSIGNED,
    block_hash          VARCHAR,
    block_number        BIGINT UNSIGNED,
    cumulative_gas_used BIGINT UNSIGNED,
    gas_used            BIGINT UNSIGNED,
    contract_address    VARCHAR,
    root                VARCHAR,
    status              BIGINT UNSIGNED,
    effective_gas_price BIGINT UNSIGNED
);

CREATE TABLE token_transfers
(
    token_address    VARCHAR,
    from_address     VARCHAR,
    to_address       VARCHAR,
    token_id         VARCHAR,
    value            DECIMAL(36, 18),
    erc_standard     VARCHAR,
    transaction_hash VARCHAR,
    log_index        BIGINT UNSIGNED,
    block_number     BIGINT UNSIGNED
);

CREATE TABLE ens
(
    name             VARCHAR,
    cost             DECIMAL(36, 18),
    expires          BIGINT UNSIGNED,
    owner            VARCHAR,
    transaction_hash VARCHAR,
    block_number     BIGINT UNSIGNED
);