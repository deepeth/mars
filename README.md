<p align="center"><b>Mars: The powerful analysis platform to explore and visualize data from Web3</b></p>

## Features

- __Blazing Fast__ Create from scratch with Rust.

- __Pipeline Processor__ Export Ethereum chain-data to structured-data in hours.

- __Low Cost__ Store structured-data to AWS S3, Azure Blob.

- __Easy to Use__ Web3 visualization and analysis at your fingertips.

## Tools

- __ethetl__ Lets you export Ethereum data into CSV/Parquet/JSON file format and databases, blazing fast.

## How to Use

```shell
$ make build

./target/release/ethetl -s 3000000 -e 3099999 -p http://192.168.191.66:8848  -o /tmp/datas/ -w 16 -f parquet
[2022-07-26T06:59:35Z INFO ] Config: Config { provider_uri: "http://192.168.191.66:8848", start_block: 3000000, end_block: 3099999, batch_size: 10000, max_worker: 16, web3_batch_size: 1000, output_dir: "/tmp/datas/", output_format: "parquet" }
[2022-07-26T06:59:41Z INFO ] 8590 blocks processed, 62662 transactions processed, 0 receipts processed. Progress is 8%
[2022-07-26T06:59:43Z INFO ] 10000 blocks processed, 77326 transactions processed, 0 receipts processed. Progress is 10%
[2022-07-26T06:59:45Z INFO ] 17000 blocks processed, 128311 transactions processed, 0 receipts processed. Progress is 17%
[2022-07-26T06:59:47Z INFO ] 21000 blocks processed, 164659 transactions processed, 0 receipts processed. Progress is 21%
[2022-07-26T06:59:49Z INFO ] 26000 blocks processed, 201764 transactions processed, 0 receipts processed. Progress is 26%
[2022-07-26T06:59:51Z INFO ] 29000 blocks processed, 222384 transactions processed, 0 receipts processed. Progress is 29%
[2022-07-26T06:59:53Z INFO ] 37000 blocks processed, 274097 transactions processed, 0 receipts processed. Progress is 37%
[2022-07-26T06:59:55Z INFO ] 41000 blocks processed, 307932 transactions processed, 0 receipts processed. Progress is 41%
[2022-07-26T06:59:57Z INFO ] 44000 blocks processed, 332340 transactions processed, 0 receipts processed. Progress is 44%
... ...

ls /tmp/datas/
3000000_3009999  3020000_3029999  3040000_3049999  3060000_3069999  3080000_3089999  3100000_3100000  3110000_3119999  3130000_3139999  3150000_3159999
3010000_3019999  3030000_3039999  3050000_3059999  3070000_3079999  3090000_3099999  3100000_3109999  3120000_3129999  3140000_3149999  3160000_3169999
```

## License

Mars is licensed under [Apache 2.0](LICENSE).
