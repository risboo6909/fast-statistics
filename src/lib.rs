#[macro_use] extern crate failure;
#[macro_use] extern crate cpython;

mod stat_funcs;
mod errors;

use std::clone::Clone;
use cpython::{PyResult, Python, PyObject, FromPyObject};
use std::thread;
use errors::{MyError, to_python_result};


#[inline]
fn pylist_to_vec<T>(py: Python, xs: PyObject) -> PyResult<Vec<T>>
    where for<'a> T: FromPyObject<'a> {
    Vec::extract(py, &xs)
}

py_module_initializer!(libfast_stat, initlibfast_stat, PyInit_libfast_stat, |py, m| {
    m.add(py, "avg_float", py_fn!(py, avg_float_py(xs: PyObject)))?;
    m.add(py, "avg_int", py_fn!(py, avg_int_py(xs: PyObject)))?;
    m.add(py, "avg_uint", py_fn!(py, avg_uint_py(xs: PyObject)))?;

    m.add(py, "harmonic_mean", py_fn!(py, harmonic_mean_py(xs: PyObject)))?;

    m.add(py, "median_float", py_fn!(py, median_float_py(xs: PyObject)))?;
    m.add(py, "median_int", py_fn!(py, median_int_py(xs: PyObject)))?;
    m.add(py, "median_uint", py_fn!(py, median_uint_py(xs: PyObject)))?;

    m.add(py, "kth_float", py_fn!(py, kth_float_py(xs: PyObject, k: usize)))?;
    m.add(py, "kth_int", py_fn!(py, kth_int_py(xs: PyObject, k: usize)))?;
    m.add(py, "kth_uint", py_fn!(py, kth_uint_py(xs: PyObject, k: usize)))?;

    Ok(())
});


// Average functions for float, int and uint

fn avg_float_py(py: Python, xs: PyObject) -> PyResult<f64> {
    let ys: Vec<f64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::avg_num(ys))
}

fn avg_int_py(py: Python, xs: PyObject) -> PyResult<i64> {
    let ys: Vec<i64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::avg_num(ys))
}

fn avg_uint_py(py: Python, xs: PyObject) -> PyResult<u64> {
    let ys: Vec<u64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::avg_num(ys))
}

// Harmonic mean has a meaning for floats only

fn harmonic_mean_py(py: Python, xs: PyObject) -> PyResult<f64> {
    let ys: Vec<f64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::harmonic_mean(ys))
}

// Median, median_low and median_high

fn median_float_py(py: Python, xs: PyObject) -> PyResult<f64> {
    let mut ys: Vec<f64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median(&mut ys))
}

fn median_int_py(py: Python, xs: PyObject) -> PyResult<i64> {
    let mut ys: Vec<i64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median(&mut ys))
}

fn median_uint_py(py: Python, xs: PyObject) -> PyResult<u64> {
    let mut ys: Vec<u64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median(&mut ys))
}

//fn median_low_float_py(py: Python, xs: PyObject) -> PyResult<f64> {
//    let ys: Vec<f64> = pylist_to_vec(py, xs)?;
//    to_python_result(py, stat_funcs::median_low(ys))
//}
//
//fn median_low_int_py(py: Python, xs: PyObject) -> PyResult<i64> {
//    let ys: Vec<i64> = pylist_to_vec(py, xs)?;
//    to_python_result(py, stat_funcs::median_low(ys))
//}
//
//fn median_low_uint_py(py: Python, xs: PyObject) -> PyResult<u64> {
//    let ys: Vec<u64> = pylist_to_vec(py, xs)?;
//    to_python_result(py, stat_funcs::median_low(ys))
//}

// k-th order statistic for float, int and uint

fn kth_float_py(py: Python, xs: PyObject, k: usize) -> PyResult<f64> {
    let mut ys = pylist_to_vec::<f64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_int_py(py: Python, xs: PyObject, k: usize) -> PyResult<i64> {
    let mut ys = pylist_to_vec::<i64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_uint_py(py: Python, xs: PyObject, k: usize) -> PyResult<u64> {
    let mut ys = pylist_to_vec::<u64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}
