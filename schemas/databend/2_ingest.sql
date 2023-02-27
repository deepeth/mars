
-- Create a external stage with your ETH files.
-- https://databend.rs/doc/sql-commands/ddl/stage/ddl-create-stage#externalstageparams
CREATE STAGE eth_stage URL='s3://<your-s3-bucket>/<eth-root>' CONNECTION = (ACCESS_KEY_ID = '<your-access-key-ID>' SECRET_ACCESS_KEY = '<your-secret-access-key>');

-- Databend provides idempotency by keeping track of files that have already been processed for a default period of 7 days.
-- https://databend.rs/doc/sql-commands/dml/dml-copy-into-table#externalstage
COPY INTO blocks FROM @eth_stage/blocks/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO transactions FROM @eth_stage/transactions/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO receipts FROM @eth_stage/receipts/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO token_transfers FROM @eth_stage/token_transfers/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO logs FROM @eth_stage/logs/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO ens FROM @eth_stage/ens/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
