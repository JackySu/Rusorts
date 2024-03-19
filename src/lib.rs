// enable portable simd feature to use simd in nightly build
#![feature(portable_simd)]
#![feature(slice_swap_unchecked)]
#![feature(maybe_uninit_uninit_array)]

extern crate rand;

pub mod qsort;
pub mod test;
pub mod ty;
pub mod util;

use qsort::*;
use ty::FloatOrd;
use util::*;

use pyo3::prelude::*;
use crumsort::ParCrumSort;
use rayon::prelude::*;


#[pyfunction]
fn f32_par_pdqsort(mut v: Vec<FloatOrd>) -> u64 {
    time_it(|| { v.par_sort_unstable() })
}

#[pyfunction]
fn f32_par_crumsort(mut v: Vec<FloatOrd>) -> u64 {
    time_it(|| v.par_crumsort())
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
    m.add_function(wrap_pyfunction!(f32_par_pdqsort, m)?)?;
    m.add_function(wrap_pyfunction!(f32_par_crumsort, m)?)?;
	m.add_function(wrap_pyfunction!(f32_1_pivot_quicksort_hoare_block_partition, m)?)?;
	m.add_function(wrap_pyfunction!(f32_4_pivot_quicksort, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", r#"Rust sorting algorithms:
    f32_par_pdqsort: pdqsort::sort(&mut [f32]) -> u64 (nanoseconds) - uses parallel pdqsort
f32_par_crumsort: std::vec::Vec<f32> -> u64 (nanoseconds) - uses parallel crumsort
f32_1_pivot_quicksort_hoare_block_partition: quick_sort_hoare_partition_block(&mut [f32]) -> u64 (nanoseconds) - uses quick_sort_hoare_partition_block
f32_4_pivot_quicksort: quad_pivot_quicksort(&mut [f32]) -> u64 (nanoseconds) - uses quad_pivot_quicksort
"#)?;
    Ok(())
}
