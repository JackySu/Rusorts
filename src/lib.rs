
extern crate rand;
extern crate introsort;
extern crate pdqsort;

pub mod util;
pub mod test;
pub mod qsort;
pub mod ty;

use pyo3::prelude::*;
use ty::FloatOrd;
use qsort::{quick_sort, double_pivot_quicksort, triple_pivot_quicksort};


#[pyfunction]
fn f32_std_sort(mut v: Vec<f32>) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
}

#[pyfunction]
fn f32_vec_rng(n: usize) -> Vec<f32> {
    util::default_vec(n)
}

#[pyfunction]
fn f32_introsort(mut v: Vec<f32>) {
    let v: &mut [FloatOrd] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut FloatOrd, v.len()) 
    };
    introsort::sort(v);
}

#[pyfunction]
fn f32_pdqsort(mut v: Vec<f32>) {
    let v: &mut [FloatOrd] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut FloatOrd, v.len()) 
    };
    pdqsort::sort(v);
}

#[pyfunction]
fn f32_quicksort(mut v: Vec<f32>) {
    let v: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut f32, v.len()) 
    };
    quick_sort(v);
}

#[pyfunction]
fn f32_double_pivot_quicksort(mut v: Vec<f32>) {
    let v: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut f32, v.len()) 
    };
    double_pivot_quicksort(v);
}

#[pyfunction]
fn f32_triple_pivot_quicksort(mut v: Vec<f32>) {
    let v: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut f32, v.len()) 
    };
    triple_pivot_quicksort(v);
}


/// A Python module implemented in Rust.
#[pymodule]
fn rust_sorts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(f32_std_sort, m)?)?;
    m.add_function(wrap_pyfunction!(f32_vec_rng, m)?)?;
    m.add_function(wrap_pyfunction!(f32_introsort, m)?)?;
    m.add_function(wrap_pyfunction!(f32_pdqsort, m)?)?;
    m.add_function(wrap_pyfunction!(f32_quicksort, m)?)?;
    m.add_function(wrap_pyfunction!(f32_double_pivot_quicksort, m)?)?;
    m.add_function(wrap_pyfunction!(f32_triple_pivot_quicksort, m)?)?;
    Ok(())
}