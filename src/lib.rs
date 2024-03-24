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
use util::*;

use pyo3::{prelude::*, types::PyList};
use crumsort::ParCrumSort;
use rayon::prelude::*;


#[pyfunction]
fn par_pdqsort(v: &PyList) -> PyResult<(Vec<i32>, u64)> {
    let mut v = pylist_to_ord_vec(v)?;
    let t = time_it(|| (&mut v).par_sort_unstable());
    Ok((v, t))
}

#[pyfunction]
fn par_crumsort(v: &PyList) -> PyResult<(Vec<i32>, u64)> {
    let mut v = pylist_to_ord_vec(v)?;
    let t = time_it(|| (&mut v).par_crumsort());
    Ok((v, t))
}

#[pyfunction]
fn single_pivot_quicksort_hoare_block_partition(v: &PyList) -> PyResult<(Vec<i32>, u64)> {
    let mut v = pylist_to_ord_vec(v)?;
    let t = time_it(|| quick_sort_hoare_partition_block(&mut v));
    Ok((v, t))
}

#[pyfunction]
fn quadro_pivot_quicksort(v: &PyList) -> PyResult<(Vec<i32>, u64)> {
    let mut v = pylist_to_ord_vec(v).unwrap();
    let t = time_it(|| quad_pivot_quicksort(&mut v));
    Ok((v, t))
}

#[pymodule]
#[pyo3(name = "rust_sorts")]
fn rust_sorts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(par_pdqsort, m)?)?;
    m.add_function(wrap_pyfunction!(par_crumsort, m)?)?;
	m.add_function(wrap_pyfunction!(single_pivot_quicksort_hoare_block_partition, m)?)?;
	m.add_function(wrap_pyfunction!(quadro_pivot_quicksort, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", r#"Rust sorting algorithms:
- ! - All algorithms return a tuple of sorted [i32] and u64 (nanoseconds) of time it takes to sort the list.
par_pdqsort: -> uses par_sort_unstable (PDQSort) boosted by Rayon
par_crumsort -> uses parallel Crumsort boosted by Rayon 
single_pivot_quicksort_hoare_block_partition -> uses quick_sort_hoare_partition_block
quadro_pivot_quicksort -> uses 4-Pivots QuickSort
"#)?;
    Ok(())
}
