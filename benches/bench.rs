#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion, measurement::WallTime};
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

macro_rules! gen_bench_for_measurement {
    ($func: ident, $typ: ty, $postfix: expr) => {
        pub fn $func(c: &mut Criterion<$typ>) {
            let mut name = String::from("1_pivot_hoare");
            name.push_str($postfix);
            let mut g1_hoare = c.benchmark_group(name);
        
            g1_hoare.bench_function("1_pivot_f32_hoare_sort_small_random", |b| 
                b.iter(|| quick_sort_hoare_partition(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // g1_hoare.bench_function("1_pivot_f32_hoare_sort_small_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // g1_hoare.bench_function("1_pivot_f32_hoare_sort_small_mostly_descending", |b| 
            //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            g1_hoare.bench_function("1_pivot_f32_hoare_sort_medium_random", |b| 
                b.iter(|| quick_sort_hoare_partition(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // g1_hoare.bench_function("1_pivot_f32_hoare_sort_medium_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // g1_hoare.bench_function("1_pivot_f32_hoare_sort_medium_mostly_descending", |b| 
            //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            g1_hoare.bench_function("1_pivot_f32_hoare_sort_large_random", |b| 
                b.iter(|| quick_sort_hoare_partition(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // g1_hoare.bench_function("1_pivot_f32_hoare_sort_large_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // g1_hoare.bench_function("1_pivot_f32_hoare_sort_large_mostly_descending", |b| 
            //     b.iter(|| quick_sort_hoare_partition(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            g1_hoare.finish();
        
            let mut name = String::from("1_pivot_lomuto");
            name.push_str($postfix);
            let mut g1_lomuto = c.benchmark_group(name);
            
            g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_small_random", |b| 
                b.iter(|| quick_sort_lomuto_partition(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_small_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_small_mostly_descending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_medium_random", |b| 
                b.iter(|| quick_sort_lomuto_partition(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_medium_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_medium_mostly_descending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_large_random", |b| 
                b.iter(|| quick_sort_lomuto_partition(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_large_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // g1_lomuto.bench_function("1_pivot_f32_lomuto_sort_large_mostly_descending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            g1_lomuto.finish();
        
            let mut name = String::from("2_pivot");
            name.push_str($postfix);
            let mut g2 = c.benchmark_group(name);
        
            g2.bench_function("2_pivot_f32_sort_small_random", |b| 
                b.iter(|| double_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // g2.bench_function("2_pivot_f32_sort_small_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // g2.bench_function("2_pivot_f32_sort_small_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            g2.bench_function("2_pivot_f32_sort_medium_random", |b| 
                b.iter(|| double_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // g2.bench_function("2_pivot_f32_sort_medium_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // g2.bench_function("2_pivot_f32_sort_medium_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            g2.bench_function("2_pivot_f32_sort_large_random", |b| 
                b.iter(|| double_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // g2.bench_function("2_pivot_f32_sort_large_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // g2.bench_function("2_pivot_f32_sort_large_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
            
            g2.finish();
        
            let mut name = String::from("3_pivot");
            name.push_str($postfix);
            let mut g3 = c.benchmark_group(name);
        
            g3.bench_function("3_pivot_f32_sort_small_random", |b| 
                b.iter(|| triple_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // g3.bench_function("3_pivot_f32_sort_small_mostly_ascending", |b| 
            //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // g3.bench_function("3_pivot_f32_sort_small_mostly_descending", |b| 
            //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            g3.bench_function("3_pivot_f32_sort_medium_random", |b| 
                b.iter(|| triple_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // g3.bench_function("3_pivot_f32_sort_medium_mostly_ascending", |b| 
            //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // g3.bench_function("3_pivot_f32_sort_medium_mostly_descending", |b| 
            //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            g3.bench_function("3_pivot_f32_sort_large_random", |b| 
                b.iter(|| triple_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // g3.bench_function("3_pivot_f32_sort_large_mostly_ascending", |b| 
            //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // g3.bench_function("3_pivot_f32_sort_large_mostly_descending", |b| 
            //     b.iter(|| triple_pivot_quicksort(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            g3.finish();
        
            let mut name = String::from("4_pivot");
            name.push_str($postfix);
            let mut g4 = c.benchmark_group(name);
        
            g4.bench_function("4_pivot_f32_sort_small_random", |b| 
                b.iter(|| quad_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // g4.bench_function("4_pivot_f32_sort_small_mostly_ascending", |b| 
            //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // g4.bench_function("4_pivot_f32_sort_small_mostly_descending", |b| 
            //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            g4.bench_function("4_pivot_f32_sort_medium_random", |b| 
                b.iter(|| quad_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // g4.bench_function("4_pivot_f32_sort_medium_mostly_ascending", |b| 
            //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // g4.bench_function("4_pivot_f32_sort_medium_mostly_descending", |b| 
            //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            g4.bench_function("4_pivot_f32_sort_large_random", |b| 
                b.iter(|| quad_pivot_quicksort(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // g4.bench_function("4_pivot_f32_sort_large_mostly_ascending", |b| 
            //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // g4.bench_function("4_pivot_f32_sort_large_mostly_descending", |b| 
            //     b.iter(|| quad_pivot_quicksort(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            g4.finish();

            let mut name = String::from("block_partition_hoare");
            name.push_str($postfix);
            let mut h = c.benchmark_group(name);
        
            h.bench_function("1_pivot_f32_hoare_small_random", |b| 
                b.iter(|| quick_sort_hoare_partition_block(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // h.bench_function("1_pivot_f32_hoare_small_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // h.bench_function("1_pivot_f32_hoare_small_mostly_descending", |b| 
            //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            h.bench_function("1_pivot_f32_hoare_medium_random", |b| 
                b.iter(|| quick_sort_hoare_partition_block(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // h.bench_function("1_pivot_f32_hoare_medium_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // h.bench_function("1_pivot_f32_hoare_medium_mostly_descending", |b| 
            //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            h.bench_function("1_pivot_f32_hoare_large_random", |b| 
                b.iter(|| quick_sort_hoare_partition_block(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // h.bench_function("1_pivot_f32_hoare_large_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // h.bench_function("1_pivot_f32_hoare_large_mostly_descending", |b| 
            //     b.iter(|| quick_sort_hoare_partition_block(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            h.finish();
        
            let mut name = String::from("block_partition_lomuto");
            name.push_str($postfix);
            let mut l = c.benchmark_group(name);
        
            l.bench_function("1_pivot_f32_lomuto_small_random", |b| 
                b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // l.bench_function("1_pivot_f32_lomuto_small_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // l.bench_function("1_pivot_f32_lomuto_small_mostly_descending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            l.bench_function("1_pivot_f32_lomuto_medium_random", |b| 
                b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // l.bench_function("1_pivot_f32_lomuto_medium_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // l.bench_function("1_pivot_f32_lomuto_medium_mostly_descending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            l.bench_function("1_pivot_f32_lomuto_large_random", |b| 
                b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // l.bench_function("1_pivot_f32_lomuto_large_mostly_ascending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // l.bench_function("1_pivot_f32_lomuto_large_mostly_descending", |b| 
            //     b.iter(|| quick_sort_lomuto_partition_block(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            l.bench_function("2_pivot_f32_lomuto_small_random", |b| 
                b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // l.bench_function("2_pivot_f32_lomuto_small_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // l.bench_function("2_pivot_f32_lomuto_small_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            l.bench_function("2_pivot_f32_lomuto_medium_random", |b| 
                b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
            // l.bench_function("2_pivot_f32_lomuto_medium_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(MEDIUM_SIZE))))
            // );
            // l.bench_function("2_pivot_f32_lomuto_medium_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_descending_f32(MEDIUM_SIZE))))
            // );
        
            l.bench_function("2_pivot_f32_lomuto_large_random", |b| 
                b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // l.bench_function("2_pivot_f32_lomuto_large_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // l.bench_function("2_pivot_f32_lomuto_large_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort_lomuto_partition_block(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            l.finish();
        
            let mut name = String::from("block_partition_new");
            name.push_str($postfix);
            let mut n = c.benchmark_group(name);
        
            n.bench_function("2_pivot_f32_new_small_random", |b| 
                b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut default_vec::<FloatOrd>(SMALL_SIZE))))
            );
            // n.bench_function("2_pivot_f32_new_small_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_ascending_f32(SMALL_SIZE))))
            // );
            // n.bench_function("2_pivot_f32_new_small_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_descending_f32(SMALL_SIZE))))
            // );
        
            n.bench_function("2_pivot_f32_new_medium_random", |b| 
                b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut default_vec::<FloatOrd>(MEDIUM_SIZE))))
            );
        
            n.bench_function("2_pivot_f32_new_large_random", |b| 
                b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut default_vec::<FloatOrd>(LARGE_SIZE))))
            );
            // n.bench_function("2_pivot_f32_new_large_mostly_ascending", |b| 
            //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_ascending_f32(LARGE_SIZE))))
            // );
            // n.bench_function("2_pivot_f32_new_large_mostly_descending", |b| 
            //     b.iter(|| double_pivot_quicksort_new_partition_block(black_box(&mut mostly_descending_f32(LARGE_SIZE))))
            // );
        
            n.finish();
        
        }
    };
}

gen_bench_for_measurement!(time_bench, WallTime, "_time");
gen_bench_for_measurement!(cpu_cycle_bench, Perf, "_cpu_cycle");
gen_bench_for_measurement!(cache_miss_bench, Perf, "_cache_miss");
gen_bench_for_measurement!(branch_miss_bench, Perf, "_branch_miss");


const SMALL_SIZE: usize = 100;
const MEDIUM_SIZE: usize = 1000;
const LARGE_SIZE: usize = 10000;

const SAMPLE_SIZE: usize = 20;


criterion_group!(
    name = time;
    config = Criterion::default().sample_size(SAMPLE_SIZE);
    targets = time_bench
);

criterion_group!(
    name = cpu_cycles;
    config = Criterion::default().sample_size(SAMPLE_SIZE).with_measurement(Perf::new(Builder::from_hardware_event(Hardware::CPUCycles)));
    targets = cpu_cycle_bench
);

criterion_group!(
    name = branch_misses;
    config = Criterion::default().sample_size(SAMPLE_SIZE).with_measurement(Perf::new(Builder::from_hardware_event(Hardware::BranchMisses)));
    targets = branch_miss_bench
);

criterion_group!(
    name = cache_misses;
    config = Criterion::default().sample_size(SAMPLE_SIZE).with_measurement(Perf::new(Builder::from_hardware_event(Hardware::CacheMisses)));
    targets = cache_miss_bench
);

criterion_main!(time, cpu_cycles, branch_misses, cache_misses);
