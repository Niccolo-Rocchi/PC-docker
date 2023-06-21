## PC-Stable benchmarking

This repository serves to benchmark PC-Stable structure learning algorithm in two different implementations. The first is provided by Rust [causal-hub](https://github.com/AlessioZanga/causal-hub) library, while the second by R [bnlearn](https://www.bnlearn.com/) library. 

Benchmarks make use of Docker and Docker Compose: make sure you have installed the required environment and have access to them. The following commands must be run in the present folder.

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

Rust results will be saved into `./Rust/criterion/` while R results will be saved into `./R/bnlearn_benches.csv`.
