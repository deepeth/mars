<p align="center">
 <b>Mars: The powerful analysis platform to explore and visualize data from Web3</b>
</p>

## Features

- **Blazing Fast:** Mars is built with Rust, which makes it incredibly fast and efficient, ensuring that you can work with your data in real-time.

- **Pipeline Processor:** Mars offers a powerful pipeline processor that allows you to export Ethereum chain data to structured data quickly and easily.

- **Low Cost:** You can store structured data to AWS S3, Azure Blob, ensuring low cost and high availability.

- **Easy to Use:** Web3 visualization and analysis at your fingertips.


## How to Use

### 1. Download the `ethetl` binary

Download the `ethetl` binary from [GitHub releases](https://github.com/deepeth/mars/tags).

### 2. Configuration

Copy the [sample config](https://github.com/deepeth/mars/blob/main/scripts/deploy/ethetl_config_spec.toml) to `mars.toml`:

```toml
[log]
level = "ERROR"
dir = "_logs"

[export]
# Exporter directory.
output_dir = "pub"

# Storage config.
[storage]
# Fs| S3 | Azblob
type = "S3"

# To use S3-compatible object storage, uncomment this block and set your values.
[storage.s3]
 bucket = "<your-bucket-name>"
 access_key_id = "<your-key-id>"
 secret_access_key = "<your-account-key>"
```
Note that the data will be stored to `/<your-bucket-name>/pub` in your S3 location.

### 3. Export Data from the Ethereum Chain by Mars

Once you have configured Mars, you can start exporting data from the Ethereum chain:
```shell
./ethetl  -p <your-eth-node-endpoint-url> -s 16600001 -e 16600002 -c ./mars.toml

... ...

[2023-02-27T08:54:07Z INFO ] block 2 processed/2, latest block 16600002, 292 transactions processed, 292 receipts processed, 658 logs processed, 329 token_transfers processed, 1 ens processed. Progress is 100%
```

Here, we recommend you use a SaaS like [GetBlock](https://getblock.io/) for your `your-eth-node-endpoint-url`.

### 4. Deploy Databend

Databend is the only warehouse supported by Mars, which has blazing performance and stores data to cloud-based object storage. 

There are two choice for you:
* Self-Deploy. See [How to deploy Databend](https://databend.rs/doc/deploy/deploying-databend)
* Cloud. Use https://app.databend.com

### 5. Create Table

You can find the schema files for Databend in the [schemas/databend](schemas/databend/1_schema.sql).

### 6. Ingest Data into Databend

Ingesting data from S3 into Databend is straightforward. You can use Databend [COPY INTO](https://databend.rs/doc/sql-commands/dml/dml-copy-into-table) command to do that:

```sql
-- Create a external stage
-- https://databend.rs/doc/sql-commands/ddl/stage/ddl-create-stage#externalstageparams
CREATE STAGE eth_stage URL='s3://<your-s3-bucket>/pub' CONNECTION = (ACCESS_KEY_ID = '<your-access-key-ID>' SECRET_ACCESS_KEY = '<your-secret-access-key>');

-- Databend provides idempotency by keeping track of files that have already been processed for a default period of 7 days
-- https://databend.rs/doc/sql-commands/dml/dml-copy-into-table#externalstage
COPY INTO blocks FROM @eth_stage/blocks/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO transactions FROM @eth_stage/transactions/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO receipts FROM @eth_stage/receipts/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO token_transfers FROM @eth_stage/token_transfers/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO logs FROM @eth_stage/logs/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
COPY INTO ens FROM @eth_stage/ens/ PATTERN = '*.*parquet' FILE_FORMAT = (type = 'PARQUET');
```

## License

Mars is licensed under [Apache 2.0](LICENSE).
