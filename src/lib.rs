#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate cpython;

mod errors;
mod stat_funcs;
#[macro_use]
mod macros;

use std::fmt::Debug;
use crate::errors::MyError;
use cpython::{FromPyObject, PyDrop, PyList, PyObject, PyResult, Python};
use crate::errors::to_python_result;
use ordered_float::OrderedFloat;
use num::{Num, FromPrimitive, Float};


py_module_initializer!(
    libfast_stat,
    initlibfast_stat,
    PyInit_libfast_stat,
    |py, m| {

        m.add(py, "mean_f64", py_fn!(py, mean_f64_py(xs: PyObject)))?;
        m.add(py, "mean_f32", py_fn!(py, mean_f32_py(xs: PyObject)))?;
        m.add(py, "variance_f64", py_fn!(py, variance_f64_py(xs: PyObject)))?;
        m.add(py, "variance_f32", py_fn!(py, variance_f32_py(xs: PyObject)))?;
        m.add(
            py,
            "harmonic_mean_f64",
            py_fn!(py, harmonic_mean_f64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "harmonic_mean_f32",
            py_fn!(py, harmonic_mean_f32_py(xs: PyObject)),
        )?;

        m.add(
            py,
            "median_f64",
            py_fn!(py, median_f64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "median_f32",
            py_fn!(py, median_f32_py(xs: PyObject)),
        )?;

        m.add(
            py,
            "median_low_f64",
            py_fn!(py, median_low_f64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "median_low_f32",
            py_fn!(py, median_low_f32_py(xs: PyObject)),
        )?;

        m.add(
            py,
            "median_high_f64",
            py_fn!(py, median_high_f64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "median_high_f32",
            py_fn!(py, median_high_f32_py(xs: PyObject)),
        )?;

        m.add(py, "mode_f64", py_fn!(py, mode_f64_py(xs: PyObject)))?;
        m.add(py, "mode_f32", py_fn!(py, mode_f32_py(xs: PyObject)))?;
        m.add(py, "mode_i64", py_fn!(py, mode_i64_py(xs: PyObject)))?;
        m.add(py, "mode_i32", py_fn!(py, mode_i32_py(xs: PyObject)))?;
        m.add(py, "mode_u64", py_fn!(py, mode_u64_py(xs: PyObject)))?;
        m.add(py, "mode_u32", py_fn!(py, mode_u32_py(xs: PyObject)))?;
        m.add(py, "mode_str", py_fn!(py, mode_str_py(xs: PyObject)))?;

        m.add(
            py,
            "kth_elem_f64",
            py_fn!(py, kth_elem_f64_py(xs: PyObject, k: usize)),
        )?;
        m.add(
            py,
            "kth_elem_f32",
            py_fn!(py, kth_elem_f32_py(xs: PyObject, k: usize)),
        )?;
        m.add(
            py,
            "kth_elem_i64",
            py_fn!(py, kth_elem_i64_py(xs: PyObject, k: usize)),
        )?;
        m.add(
            py,
            "kth_elem_i32",
            py_fn!(py, kth_elem_i32_py(xs: PyObject, k: usize)),
        )?;
        m.add(
            py,
            "kth_elem_u64",
            py_fn!(py, kth_elem_u64_py(xs: PyObject, k: usize)),
        )?;
        m.add(
            py,
            "kth_elem_u32",
            py_fn!(py, kth_elem_u32_py(xs: PyObject, k: usize)),
        )?;

        Ok(())
    }
);

#[inline]
fn pylist_to_vec<T>(py: Python<'_>, xs: PyObject) -> PyResult<Vec<T>>
where for<'a> T: FromPyObject<'a> {
    Vec::extract(py, &xs)
}

#[inline]
fn extract_ordered_floats<T>(py: Python<'_>, obj: &PyObject) -> PyResult<Vec<OrderedFloat<T>>> where 
    for<'a> T: Float + FromPyObject<'a> {

    let list = try!(obj.cast_as::<PyList>(py));

    let len = list.len(py);
    let mut v = Vec::with_capacity(len);

    for i in 0..len {
        let item = list.get_item(py, i);
        v.push(OrderedFloat(T::extract(py, &item)?));
        item.release_ref(py);
    }

    Ok(v)
}

expander!(variance, (variance_f64_py, f64), (variance_f32_py, f32));

expander!(mean, (mean_f64_py, f64), (mean_f32_py, f32));

expander!(harmonic_mean, (harmonic_mean_f64_py, f64), (harmonic_mean_f32_py, f32));

expander_mut!(median, (median_f64_py, f64), (median_f32_py, i64));

// floats have to be converted to OrderedFloats explicitly,
// therefor can't be expanded with macros
fn median_low_f64_py(py: Python<'_>, xs: PyObject) -> PyResult<f64> {
    let mut ys = extract_ordered_floats(py, &xs)?;
    let res = match stat_funcs::median_low::<OrderedFloat<f64>>(&mut ys) {
        Ok(res) => Ok(res.into()),
        Err(err) => Err(err),
    };

    to_python_result(py, res)
}

// floats have to be converted to OrderedFloats explicitly,
// therefor can't be expanded with macros
fn median_low_f32_py(py: Python<'_>, xs: PyObject) -> PyResult<f32> {
    let mut ys = extract_ordered_floats(py, &xs)?;
    let res = match stat_funcs::median_low::<OrderedFloat<f32>>(&mut ys) {
        Ok(res) => Ok(res.into()),
        Err(err) => Err(err),
    };

    to_python_result(py, res)
}

// floats have to be converted to OrderedFloats explicitly,
// therefor can't be expanded with macros
fn median_high_f64_py(py: Python<'_>, xs: PyObject) -> PyResult<f64> {
    let mut ys = extract_ordered_floats(py, &xs)?;
    let res = match stat_funcs::median_high::<OrderedFloat<f64>>(&mut ys) {
        Ok(res) => Ok(res.into()),
        Err(err) => Err(err),
    };

    to_python_result(py, res)
}

// floats have to be converted to OrderedFloats explicitly,
// therefor can't be expanded with macros
fn median_high_f32_py(py: Python<'_>, xs: PyObject) -> PyResult<f32> {
    let mut ys = extract_ordered_floats(py, &xs)?;
    let res = match stat_funcs::median_high::<OrderedFloat<f32>>(&mut ys) {
        Ok(res) => Ok(res.into()),
        Err(err) => Err(err),
    };

    to_python_result(py, res)
}


// floats have to be converted to OrderedFloats explicitly,
// therefor can't be expanded with macros
fn mode_f64_py(py: Python<'_>, xs: PyObject) -> PyResult<f64> {
    let ys = extract_ordered_floats(py, &xs)?;
    let res = match stat_funcs::mode::<OrderedFloat<f64>>(ys) {
        Ok(res) => Ok(res.into()),
        Err(err) => Err(err),
    };
    to_python_result(py, res)
}

fn mode_f32_py(py: Python<'_>, xs: PyObject) -> PyResult<f32> {
    let ys = extract_ordered_floats(py, &xs)?;
    let res = match stat_funcs::mode::<OrderedFloat<f32>>(ys) {
        Ok(res) => Ok(res.into()),
        Err(err) => Err(err),
    };
    to_python_result(py, res)
}

expander!(mode,
         (mode_str_py, String), (mode_i64_py, i64), (mode_i32_py, i32), (mode_u64_py, u64),
         (mode_u32_py, u32));

// TODO: How to fold these guys?
fn kth_elem_f64_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<f64> {
    let mut ys = pylist_to_vec::<f64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_elem_f32_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<f32> {
    let mut ys = pylist_to_vec::<f32>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_elem_u64_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<u64> {
    let mut ys = pylist_to_vec::<u64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_elem_u32_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<u32> {
    let mut ys = pylist_to_vec::<u32>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_elem_i64_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<i64> {
    let mut ys = pylist_to_vec::<i64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_elem_i32_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<i32> {
    let mut ys = pylist_to_vec::<i32>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}
