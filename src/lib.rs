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
use ty::*;
use util::*;

use crumsort::ParCrumSort;
use pyo3::{prelude::*, types::PyList};
use rayon::prelude::*;


#[pyfunction]
fn par_pdqsort<'a>(py: Python<'a>, v: &PyList) -> PyResult<(&'a PyList, u64)> {
    let v = v.extract::<OrdNum>()?;
    sort_ordnum_f!(trait, v, par_sort_unstable, py)
}

#[pyfunction]
fn par_crumsort<'a>(py: Python<'a>, v: &PyList) -> PyResult<(&'a PyList, u64)> {
    let v = v.extract::<OrdNum>()?;
    sort_ordnum_f!(trait, v, par_crumsort, py)
}

#[pyfunction]
fn single_pivot_quicksort_hoare_block_partition<'a>(
    py: Python<'a>,
    v: &PyList,
) -> PyResult<(&'a PyList, u64)> {
    let v = v.extract::<OrdNum>()?;
    sort_ordnum_f!(slice, v, quick_sort_hoare_partition_block, py)
}

#[pyfunction]
fn quadro_pivot_quicksort<'a>(
    py: Python<'a>,
    v: &PyList
) -> PyResult<(&'a PyList, u64)> {
    let v = v.extract::<OrdNum>()?;
    sort_ordnum_f!(slice, v, quad_pivot_quicksort, py)
}

#[pymodule]
#[pyo3(name = "rust_sorts")]
fn rust_sorts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(par_pdqsort, m)?)?;
    m.add_function(wrap_pyfunction!(par_crumsort, m)?)?;
    m.add_function(wrap_pyfunction!(single_pivot_quicksort_hoare_block_partition, m)?)?;
    m.add_function(wrap_pyfunction!(quadro_pivot_quicksort, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", r#"Rust sorting algorithms, ffi by PyO3
"""
Params:
    v (list[float | int]): List of nums to be sorted
Returns:
    (list[float | int], int): Sorted list and the cost time
Functions available:
par_pdqsort: -> uses par_sort_unstable (PDQSort) boosted by Rayon
par_crumsort -> uses parallel Crumsort boosted by Rayon 
single_pivot_quicksort_hoare_block_partition -> uses quick_sort_hoare_partition_block
quadro_pivot_quicksort -> uses 4-Pivots QuickSort
TODO: Add more supported types
"""
"#)?;
    Ok(())
}
