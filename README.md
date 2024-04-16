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

See `Conclusion` of [Report](./FYPReport.pdf)

> • The improvements achieved through the utilization of multiple pivots have seemingly
> reached a plateau. Despite the 4-Pivot method demonstrating the most outperforming
> runtime costs, the incremental gains do not compensate for the increased complexity.
>
> • In the context of modern CPU cache coordination, most of the cache misses have
> been eliminated for QuickSort algorithms, with branch misses emerging as the primary
> bottleneck. The Block Partition method shows significant potential in mitigating branch
> misses. 1-Pivot Hoare’s Block Partition ranks as the top performer among the tested
> algorithms, underscoring the importance of a well-optimized layout over the sheer
> number of pivots. The reduction in branch misses achieved by the Block Partition
> method indicates a promising future, though challenges of complexities brought by
> increased pivots count still remain.
> 
> • Skewed pivot selection demonstrates potential benefits in classical QuickSorts, but
> a balanced pivot selection works better in BlockQuickSort. Closer the pivot is to the
> median, better the performance.
> 
> • The novel layout of 2-Pivot Lomuto’s Block Partition exposed the limitations
> introduced by more pivots. The pattern of comparing with one pivot at a time is still
> the only way to go by far, otherwise, any deviation from this pattern will pose risks to
> substantial memory access overheads, impeding CPU performance — a critical lesson
> for future research endeavors.

### ⚠ Wrapper type [`FloatOrd`](src/ty.rs) is to fix IEEE-754's NaN, -INF, INF, -0 and 0 ordinary issues and it is USER's call to ensure the validity of data
