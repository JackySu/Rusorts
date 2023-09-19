extern crate rand;
extern crate introsort;
extern crate pdqsort;

pub mod util;
pub mod test;
pub mod qsort;
pub mod ty;

use ty::FloatOrd;
use qsort::{quick_sort, double_pivot_quicksort, triple_pivot_quicksort};

use cpython::{py_module_initializer, py_fn, Python, PyResult, PyObject};

fn f32_std_sort(py: Python, mut v: Vec<f32>) -> PyResult<PyObject> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(py.None())
}

fn f32_vec_rng(_py: Python, n: usize) -> PyResult<Vec<f32>> {
    Ok(util::default_vec(n))
}

fn f32_introsort(py: Python, mut v: Vec<f32>) -> PyResult<PyObject> {
    let v: &mut [FloatOrd] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut FloatOrd, v.len()) 
    };
    introsort::sort(v);
    Ok(py.None())
}

fn f32_pdqsort(py: Python, mut v: Vec<f32>) -> PyResult<PyObject> {
    let v: &mut [FloatOrd] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut FloatOrd, v.len()) 
    };
    pdqsort::sort(v);
    Ok(py.None())
}

fn f32_quicksort(py: Python, mut v: Vec<f32>) -> PyResult<PyObject> {
    let v: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut f32, v.len()) 
    };
    quick_sort(v);
    Ok(py.None())
}

fn f32_double_pivot_quicksort(py: Python, mut v: Vec<f32>) -> PyResult<PyObject> {
    let v: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut f32, v.len()) 
    };
    double_pivot_quicksort(v);
    Ok(py.None())
}

fn f32_triple_pivot_quicksort(py: Python, mut v: Vec<f32>) -> PyResult<PyObject> {
    let v: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut f32, v.len()) 
    };
    triple_pivot_quicksort(v);
    Ok(py.None())
}

py_module_initializer!(rust_sorts, |py, m| {
    m.add(py, "__doc__", r#"
This module is implemented in Rust to sort a vector of f32s.
other data types are not available yet but will be in the future.
- f32_std_sort: uses the standard library's sort function
- f32_vec_rng: generates a vector of random f32s
- f32_introsort: uses the introsort crate to sort
- f32_pdqsort: uses the pdqsort crate to sort
- f32_quicksort: uses a custom quicksort implementation
- f32_double_pivot_quicksort: uses a custom double pivot quicksort implementation
- f32_triple_pivot_quicksort: uses a custom triple pivot quicksort implementation

to use this module, import it and call the functions like so:
```python
import rust_sorts
rust_sorts.f32_std_sort([1.0, 2.0, 3.0])
```
    "#)?;
    m.add(py, "f32_std_sort", py_fn!(py, f32_std_sort(v: Vec<f32>)))?;
    m.add(py, "f32_vec_rng", py_fn!(py, f32_vec_rng(n: usize)))?;
    m.add(py, "f32_introsort", py_fn!(py, f32_introsort(v: Vec<f32>)))?;
    m.add(py, "f32_pdqsort", py_fn!(py, f32_pdqsort(v: Vec<f32>)))?;
    m.add(py, "f32_quicksort", py_fn!(py, f32_quicksort(v: Vec<f32>)))?;
    m.add(py, "f32_double_pivot_quicksort", py_fn!(py, f32_double_pivot_quicksort(v: Vec<f32>)))?;
    m.add(py, "f32_triple_pivot_quicksort", py_fn!(py, f32_triple_pivot_quicksort(v: Vec<f32>)))?;
    Ok(())
});