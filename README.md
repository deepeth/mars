<p align="center"><b>Mars: The powerful analysis platform to explore and visualize data from Web3</b></p>

## Features

- __Blazing Fast__ Create from scratch with Rust.

- __Pipeline Processor__ Export Ethereum chain-data to structured-data in hours.

- __Low Cost__ Store structured-data to AWS S3, Azure Blob.

- __Easy to Use__ Web3 visualization and analysis at your fingertips.

## ethetl

- __ethetl__ Lets you export Ethereum data into CSV/Parquet/JSON file format and databases, blazing fast.

### Schema

Databend:
https://github.com/deepeth/mars/tree/main/schemas/databend

Snowflake:
https://github.com/deepeth/mars/tree/main/schemas/snowflake


### How to Use

#### ethetl-stream

`ethetl-stream` will continuously fetch data from Ethereum node and transfrom the chain data to parquet files.

```shell
$ make build

ubuntu@ip-172-31-45-138:/data1/github/mars$ ./target/release/ethetl-stream -p http://127.0.0.1:8545 -s 1
[2022-11-26T06:44:56Z WARN ] collect: No such file or directory (os error 2)
[2022-11-26T06:44:56Z INFO ] Config: EthConfig { log: LogConfig { level: "INFO", dir: "_logs" }, export: ExportConfig { provider_uri: "http://127.0.0.1:8545", start_block: 1, end_block: 10000, batch_size: 1000, max_worker: 8, web3_batch_size: 100, syncing_interval_secs: 1, output_dir: "_datas", output_format: "parquet" }, storage: StorageConfig { storage_type: Fs, fs: FsStorageConfig { data_path: "./_datas" }, s3: S3StorageConfig { endpoint_url: "https://s3.amazonaws.com", region: "", bucket: "", root: "", access_key_id: "", secret_access_key: "", enable_virtual_host_style: true }, azblob: AzureStorageBlobConfig { endpoint_url: "", container: "", root: "", account_name: "", account_key: "" } }, config_file: "" }
[2022-11-26T06:44:56Z INFO ] Found syncing status file=mars_syncing_status.json, status=SyncingStatus { start: 1, end: 8000 }
[2022-11-26T06:44:57Z INFO ] eth.syncing, currentBlock=12679194, highestBlock=16051868
[2022-11-26T06:44:57Z INFO ] Syncing batch, range=[8001, 12679194], chunk_size=8000, chunks=1584
[2022-11-26T06:44:57Z INFO ] Syncing batch[0], status=SyncingStatus { start: 8001, end: 16000 }
[2022-11-26T06:44:58Z INFO ] 2500 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 25%
[2022-11-26T06:45:00Z INFO ] 2500 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 25%
[2022-11-26T06:45:02Z INFO ] 2500 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 25%
[2022-11-26T06:45:04Z INFO ] 2500 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 25%
[2022-11-26T06:45:06Z INFO ] 2800 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 28%
[2022-11-26T06:45:08Z INFO ] 3300 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 33%
[2022-11-26T06:45:10Z INFO ] 3300 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 33%
[2022-11-26T06:45:12Z INFO ] 3300 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 33%
[2022-11-26T06:45:14Z INFO ] 3300 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 33%
[2022-11-26T06:45:16Z INFO ] 3300 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 33%
[2022-11-26T06:45:18Z INFO ] 3300 blocks processed, 0 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 33%
... ...
```

## License

Mars is licensed under [Apache 2.0](LICENSE).
