## PC-Stable benchmarking

This repository serves to benchmark PC-Stable structure learning algorithm in two different implementations. The first is provided by Rust [causal-hub](https://github.com/AlessioZanga/causal-hub) library, while the second by R [bnlearn](https://www.bnlearn.com/) library. 

Benchmarks make use of Docker and Docker Compose: make sure you have installed the required packages and have access to them. All the following commands must be run in the present folder. Please pay attention to possible out-of-memory issues with the largest datasets.

To bench both implementations, run:
```
docker compose up -d
```

To bench only a single implementation, run one of the following commands:
```
docker compose run --rm -d rust-bench
```
```
docker compose run --rm -d r-bench
```

Rust benchmarking results will be saved into `./Rust/criterion/` while R benchmarking results will be saved into `./R/bnlearn_benches.csv`.
