#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion_perf_events::Perf;
use perfcnt::linux::HardwareEventType as Hardware;
use perfcnt::linux::PerfCounterBuilderLinux as Builder;

use rust_sorts::ty::FloatOrd;
use rust_sorts::{qsort::*, util::default_vec};

use rand::{Rng, thread_rng};

macro_rules! gen_mostly_ascending {
    ($func: ident, $typ: ty) => {
        fn $func(len: usize) -> Vec<$typ> {
            let mut rng = thread_rng();
            let mut v: Vec<$typ> = (0..len).map(|x| <$typ>::from(x)).collect();
            for _ in (0usize..).take_while(|x| x * x <= len) {
                let x = rng.gen::<usize>() % len;
                let y = rng.gen::<usize>() % len;
                v.swap(x, y);
            }
            v
        }
    };
}

macro_rules! gen_mostly_descending {
    ($func: ident, $typ: ty) => {
        fn $func(len: usize) -> Vec<$typ> {
            let mut rng = thread_rng();
            let mut v: Vec<$typ> = (0..len).rev().map(|x| <$typ>::from(x)).collect();
            for _ in (0usize..).take_while(|x| x * x <= len) {
                let x = rng.gen::<usize>() % len;
                let y = rng.gen::<usize>() % len;
                v.swap(x, y);
            }
            v
        }
    };
}

gen_mostly_ascending!(mostly_ascending_f32, FloatOrd);
gen_mostly_descending!(mostly_descending_f32, FloatOrd);

pub fn normal_partition_benchmark(c: &mut Criterion<Perf>) {
    let mut g1_hoare = c.benchmark_group("1_pivot_hoare");

    g1_hoare.bench_function("1_pivot_f32_hoare_sort_small_random", |b| 
        b.iter(|| quick_sort_hoare_partition(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // g1_hoare.bench_function("1_pivot_f32_hoare_sort_small_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // g1_hoare.bench_function("1_pivot_f32_hoare_sort_small_mostly_descending", |b| 
    //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_descending_f32(1_00))))
    // );

    g1_hoare.bench_function("1_pivot_f32_hoare_sort_medium_random", |b| 
        b.iter(|| quick_sort_hoare_partition(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // g1_hoare.bench_function("1_pivot_f32_hoare_sort_medium_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // g1_hoare.bench_function("1_pivot_f32_hoare_sort_medium_mostly_descending", |b| 
    //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_descending_f32(10_000))))
    // );

    g1_hoare.bench_function("1_pivot_f32_hoare_sort_large_random", |b| 
        b.iter(|| quick_sort_hoare_partition(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // g1_hoare.bench_function("1_pivot_f32_hoare_sort_large_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // g1_hoare.bench_function("1_pivot_f32_hoare_sort_large_mostly_descending", |b| 
    //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    g1_hoare.finish();

    let mut g1_lomuto = c.benchmark_group("1_pivot_lomuto");
    
    g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_small_random", |b| 
        b.iter(|| quick_sort_lomuto_partition(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_small_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_small_mostly_descending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_descending_f32(1_00))))
    // );

    g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_medium_random", |b| 
        b.iter(|| quick_sort_lomuto_partition(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_medium_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_medium_mostly_descending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_descending_f32(10_000))))
    // );

    g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_large_random", |b| 
        b.iter(|| quick_sort_lomuto_partition(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_large_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_large_mostly_descending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    g1_lomuto.finish();

    let mut g2 = c.benchmark_group("2_pivot");

    g2.bench_function("2_pivot_f32_sort_small_random", |b| 
        b.iter(|| double_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // g2.bench_function("2_pivot_f32_sort_small_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // g2.bench_function("2_pivot_f32_sort_small_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_descending_f32(1_00))))
    // );

    g2.bench_function("2_pivot_f32_sort_medium_random", |b| 
        b.iter(|| double_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // g2.bench_function("2_pivot_f32_sort_medium_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // g2.bench_function("2_pivot_f32_sort_medium_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_descending_f32(10_000))))
    // );

    g2.bench_function("2_pivot_f32_sort_large_random", |b| 
        b.iter(|| double_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // g2.bench_function("2_pivot_f32_sort_large_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // g2.bench_function("2_pivot_f32_sort_large_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_descending_f32(10_000_000))))
    // );
    
    g2.finish();

    let mut g3 = c.benchmark_group("3_pivot");

    g3.bench_function("3_pivot_f32_sort_small_random", |b| 
        b.iter(|| triple_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // g3.bench_function("3_pivot_f32_sort_small_mostly_ascending", |b| 
    //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // g3.bench_function("3_pivot_f32_sort_small_mostly_descending", |b| 
    //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_descending_f32(1_00))))
    // );

    g3.bench_function("3_pivot_f32_sort_medium_random", |b| 
        b.iter(|| triple_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // g3.bench_function("3_pivot_f32_sort_medium_mostly_ascending", |b| 
    //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // g3.bench_function("3_pivot_f32_sort_medium_mostly_descending", |b| 
    //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_descending_f32(10_000))))
    // );

    g3.bench_function("3_pivot_f32_sort_large_random", |b| 
        b.iter(|| triple_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // g3.bench_function("3_pivot_f32_sort_large_mostly_ascending", |b| 
    //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // g3.bench_function("3_pivot_f32_sort_large_mostly_descending", |b| 
    //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    g3.finish();

    let mut g4 = c.benchmark_group("4_pivot");

    g4.bench_function("4_pivot_f32_sort_small_random", |b| 
        b.iter(|| quad_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // g4.bench_function("4_pivot_f32_sort_small_mostly_ascending", |b| 
    //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // g4.bench_function("4_pivot_f32_sort_small_mostly_descending", |b| 
    //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_descending_f32(1_00))))
    // );

    g4.bench_function("4_pivot_f32_sort_medium_random", |b| 
        b.iter(|| quad_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // g4.bench_function("4_pivot_f32_sort_medium_mostly_ascending", |b| 
    //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // g4.bench_function("4_pivot_f32_sort_medium_mostly_descending", |b| 
    //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_descending_f32(10_000))))
    // );

    g4.bench_function("4_pivot_f32_sort_large_random", |b| 
        b.iter(|| quad_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // g4.bench_function("4_pivot_f32_sort_large_mostly_ascending", |b| 
    //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // g4.bench_function("4_pivot_f32_sort_large_mostly_descending", |b| 
    //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    g4.finish();
}

pub fn block_partition_benchmark(c: &mut Criterion<Perf>) {
    let mut h = c.benchmark_group("block_partition_hoare");

    h.bench_function("1_pivot_f32_hoare_small_random", |b| 
        b.iter(|| quick_sort_hoare_partition_block(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // h.bench_function("1_pivot_f32_hoare_small_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // h.bench_function("1_pivot_f32_hoare_small_mostly_descending", |b| 
    //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_descending_f32(1_00))))
    // );

    h.bench_function("1_pivot_f32_hoare_medium_random", |b| 
        b.iter(|| quick_sort_hoare_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // h.bench_function("1_pivot_f32_hoare_medium_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // h.bench_function("1_pivot_f32_hoare_medium_mostly_descending", |b| 
    //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_descending_f32(10_000))))
    // );

    h.bench_function("1_pivot_f32_hoare_large_random", |b| 
        b.iter(|| quick_sort_hoare_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // h.bench_function("1_pivot_f32_hoare_large_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // h.bench_function("1_pivot_f32_hoare_large_mostly_descending", |b| 
    //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    h.finish();

    let mut l = c.benchmark_group("block_partition_lomuto");

    l.bench_function("1_pivot_f32_lomuto_small_random", |b| 
        b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // l.bench_function("1_pivot_f32_lomuto_small_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // l.bench_function("1_pivot_f32_lomuto_small_mostly_descending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_descending_f32(1_00))))
    // );

    l.bench_function("1_pivot_f32_lomuto_medium_random", |b| 
        b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // l.bench_function("1_pivot_f32_lomuto_medium_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // l.bench_function("1_pivot_f32_lomuto_medium_mostly_descending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_descending_f32(10_000))))
    // );

    l.bench_function("1_pivot_f32_lomuto_large_random", |b| 
        b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // l.bench_function("1_pivot_f32_lomuto_large_mostly_ascending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // l.bench_function("1_pivot_f32_lomuto_large_mostly_descending", |b| 
    //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    l.bench_function("2_pivot_f32_lomuto_small_random", |b| 
        b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // l.bench_function("2_pivot_f32_lomuto_small_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // l.bench_function("2_pivot_f32_lomuto_small_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_descending_f32(1_00))))
    // );

    l.bench_function("2_pivot_f32_lomuto_medium_random", |b| 
        b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // l.bench_function("2_pivot_f32_lomuto_medium_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // l.bench_function("2_pivot_f32_lomuto_medium_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_descending_f32(10_000))))
    // );

    l.bench_function("2_pivot_f32_lomuto_large_random", |b| 
        b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // l.bench_function("2_pivot_f32_lomuto_large_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // l.bench_function("2_pivot_f32_lomuto_large_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    l.finish();

    let mut n = c.benchmark_group("block_partition_new");

    n.bench_function("2_pivot_f32_new_small_random", |b| 
        b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut default_vec::<FloatOrd>(1_00))))
    );
    // n.bench_function("2_pivot_f32_new_small_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_ascending_f32(1_00))))
    // );
    // n.bench_function("2_pivot_f32_new_small_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_descending_f32(1_00))))
    // );

    n.bench_function("2_pivot_f32_new_medium_random", |b| 
        b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000))))
    );
    // n.bench_function("2_pivot_f32_new_medium_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_ascending_f32(10_000))))
    // );
    // n.bench_function("2_pivot_f32_new_medium_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_descending_f32(10_000))))
    // );

    n.bench_function("2_pivot_f32_new_large_random", |b| 
        b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut default_vec::<FloatOrd>(10_000_000))))
    );
    // n.bench_function("2_pivot_f32_new_large_mostly_ascending", |b| 
    //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_ascending_f32(10_000_000))))
    // );
    // n.bench_function("2_pivot_f32_new_large_mostly_descending", |b| 
    //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_descending_f32(10_000_000))))
    // );

    n.finish();

}


criterion_group!(
    name = benches;
    // uncomment the line below to run the benchmarks with the default criterion configuration
    // config = Criterion::default().sample_size(10).with_measurement(Perf::new(Builder::from_hardware_event(Hardware::Instructions)));
    // config = Criterion::default().sample_size(10).with_measurement(Perf::new(Builder::from_hardware_event(Hardware::CacheMisses)));
    // config = Criterion::default().sample_size(10).with_measurement(Perf::new(Builder::from_hardware_event(Hardware::BranchMisses)));
    config = Criterion::default().sample_size(10).with_measurement(Perf::new(Builder::from_hardware_event(Hardware::CPUCycles)));
    targets = normal_partition_benchmark, block_partition_benchmark
);
// criterion_group!(name = benches;
//     config = Criterion::default().sample_size(10);
//     targets = normal_partition_benchmark, block_partition_benchmark);
criterion_main!(benches);