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


### How to Use

#### ethetl

```shell
$ make build

./target/release/ethetl  -p <your-eth-node-endpoint-url> -s 16600001 -e 16600002
[2023-02-27T08:53:57Z WARN ] collect: No such file or directory (os error 2)
[2023-02-27T08:54:02Z INFO ] Write blocks to _datas/blocks/blocks_16600001_16600002.parquet
[2023-02-27T08:54:02Z INFO ] Write transactions to _datas/transactions/transactions_16600001_16600002.parquet
[2023-02-27T08:54:02Z INFO ] Write _datas/transactions/_transactions_hash_16600001_16600002.txt
[2023-02-27T08:54:03Z INFO ] block 2 processed/2, latest block 16600002, 292 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2023-02-27T08:54:05Z INFO ] block 2 processed/2, latest block 16600002, 292 transactions processed, 100 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2023-02-27T08:54:07Z INFO ] Write receipts to _datas/receipts/receipts_16600001_16600002.parquet
[2023-02-27T08:54:07Z INFO ] Write logs to _datas/logs/logs_16600001_16600002.parquet
[2023-02-27T08:54:07Z INFO ] Write token_transfer to _datas/token_transfers/token_transfers_16600001_16600002.parquet
[2023-02-27T08:54:07Z INFO ] Write ens to _datas/ens/ens_16600001_16600002.parquet
[2023-02-27T08:54:07Z INFO ] block 2 processed/2, latest block 16600002, 292 transactions processed, 292 receipts processed, 658 logs processed, 329 token_transfers processed, 1 ens processed. Progress is 100%
```

## License

Mars is licensed under [Apache 2.0](LICENSE).
