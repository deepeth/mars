<p align="center">The platform to explore data from Ethereum</p>

## Features

- __Blazing Fast__ Create from scratch with Rust.

## How to Use

```shell
$ make build

time ./target/release/mars --start-block 50000 --end-block 100005 --provider-uri http://127.0.0.1:8848 --max-worker 16
[2022-07-05T07:14:41Z INFO  mars] Config: Config { provider_uri: "http://127.0.0.1:8848", start_block: 50000, end_block: 100005, batch_size: 1000, max_worker: 16 }
[2022-07-05T07:14:43Z INFO  mars::workers::progress] 11500 blocks processed, 4464 transactions processed, 0 receipts processed. Progress is 22%
[2022-07-05T07:14:45Z INFO  mars::workers::progress] 21000 blocks processed, 7732 transactions processed, 5631 receipts processed. Progress is 41%
[2022-07-05T07:14:47Z INFO  mars::workers::progress] 31000 blocks processed, 15699 transactions processed, 10389 receipts processed. Progress is 61%
[2022-07-05T07:14:49Z INFO  mars::workers::progress] 44000 blocks processed, 22021 transactions processed, 17280 receipts processed. Progress is 87%
[2022-07-05T07:14:51Z INFO  mars::workers::progress] 50006 blocks processed, 25100 transactions processed, 25100 receipts processed. Progress is 100%

real	0m9.605s
```

## License

Mars is licensed under [Apache 2.0](LICENSE).