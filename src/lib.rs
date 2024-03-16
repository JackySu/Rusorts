// enable portable simd feature to use simd in nightly build
#![feature(portable_simd)]
#![feature(slice_swap_unchecked)]
#![feature(maybe_uninit_uninit_array)]

extern crate pdqsort;
extern crate rand;

pub mod qsort;
pub mod test;
pub mod ty;
pub mod util;

use qsort::*;
use ty::FloatOrd;
use util::*;

use pyo3::prelude::*;


#[pyfunction]
fn f32_std_unstable_sort(mut v: Vec<FloatOrd>) -> u64 {
    time_it(|| v.sort_unstable())
}

#[pyfunction]
fn f32_pdq_sort(mut v: Vec<FloatOrd>) -> u64 {
    time_it(|| { pdqsort::sort(&mut v) })
}

#[pyfunction]
fn f32_1_pivot_quicksort_hoare_block_partition(mut v: Vec<FloatOrd>) -> u64 {
    time_it(|| quick_sort_hoare_partition_block(&mut v))
}

#[pyfunction]
fn f32_4_pivot_quicksort(mut v: Vec<FloatOrd>) -> u64 {
    time_it(|| quad_pivot_quicksort(&mut v))
}

#[pymodule]
#[pyo3(name = "rust_sorts")]
fn rust_sorts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(f32_std_unstable_sort, m)?)?;
    m.add_function(wrap_pyfunction!(f32_pdq_sort, m)?)?;
	m.add_function(wrap_pyfunction!(f32_1_pivot_quicksort_hoare_block_partition, m)?)?;
	m.add_function(wrap_pyfunction!(f32_4_pivot_quicksort, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", r#"Rust sorting algorithms:
f32_std_unstable_sort: std::vec::Vec<f32> -> u64 (nanoseconds) - uses std::vec::Vec::sort_unstable
f32_pdq_sort: pdqsort::sort(&mut [f32]) -> u64 (nanoseconds) - uses pdqsort::sort
f32_1_pivot_quicksort_hoare_block_partition: quick_sort_hoare_partition_block(&mut [f32]) -> u64 (nanoseconds) - uses quick_sort_hoare_partition_block
f32_4_pivot_quicksort: quad_pivot_quicksort(&mut [f32]) -> u64 (nanoseconds) - uses quad_pivot_quicksort
"#)?;
    Ok(())
}
