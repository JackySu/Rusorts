# Contributions

- Implemented ...
  * 1-4 Pivot Quicksort
  * 1-2 Pivot Block Partitioning Quicksort

# Run

1. run only the bench in rust

- install Rust🦀 and [`perf`](https://oopsmonk.github.io/posts/2022-04-28-perf/) for profiling

```shell
sudo sh -c 'echo 1 >/proc/sys/kernel/perf_event_paranoid'
cargo bench
```

- see `benches/bench.rs` for more details

2. install it to Python🐍

- activate virtual env
  ```shell
  python3 -m venv .venv
  source .venv/bin/activate
  ```

- build wheels
  ```
  pip install maturin .
  maturin develop --release
  ```

- see example usages in `rust_sorts/sorts.py`

# Latest results

PDQSort >= std_unstable_sort > hoare_block_partition >> 4 Pivots QSort

![](sorts.png)

### ⚠ Wrapper type [`FloatOrd`](src/ty.rs) is to fix IEEE-754's NaN, -INF, INF, -0 and 0 ordinary issues and it is USER's call to ensure the validity of data
