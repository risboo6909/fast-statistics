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
use ordered_float::*;

py_module_initializer!(
    libfast_stat,
    initlibfast_stat,
    PyInit_libfast_stat,
    |py, m| {

        m.add(py, "mean_float", py_fn!(py, mean_f64_py(xs: PyObject)))?;
        m.add(py, "mean_int", py_fn!(py, mean_i64_py(xs: PyObject)))?;
        m.add(py, "mean_uint", py_fn!(py, mean_u64_py(xs: PyObject)))?;

        m.add(py, "variance_float", py_fn!(py, variance_f64_py(xs: PyObject)))?;

        m.add(
            py,
            "harmonic_mean_float64",
            py_fn!(py, harmonic_mean_f64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "harmonic_mean_float32",
            py_fn!(py, harmonic_mean_f32_py(xs: PyObject)),
        )?;

        m.add(
            py,
            "median_float",
            py_fn!(py, median_f64_py(xs: PyObject)),
        )?;
        m.add(py, "median_int", py_fn!(py, median_i64_py(xs: PyObject)))?;
        m.add(py, "median_uint", py_fn!(py, median_u64_py(xs: PyObject)))?;

        m.add(
            py,
            "median_low_float",
            py_fn!(py, median_low_f64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "median_low_int",
            py_fn!(py, median_low_i64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "median_low_uint",
            py_fn!(py, median_low_u64_py(xs: PyObject)),
        )?;

        m.add(
            py,
            "median_high_float",
            py_fn!(py, median_high_f64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "median_high_int",
            py_fn!(py, median_high_i64_py(xs: PyObject)),
        )?;
        m.add(
            py,
            "median_high_uint",
            py_fn!(py, median_high_u64_py(xs: PyObject)),
        )?;

        m.add(py, "mode_float", py_fn!(py, mode_float_py(xs: PyObject)))?;
        m.add(py, "mode_int", py_fn!(py, mode_i64_py(xs: PyObject)))?;
        m.add(py, "mode_uint", py_fn!(py, mode_u64_py(xs: PyObject)))?;
        m.add(py, "mode_str", py_fn!(py, mode_str_py(xs: PyObject)))?;

        //    m.add(py, "kth_element", py_fn!(py, kth_py(xs: PyObject, k: usize)))?;

        m.add(
            py,
            "kth_element_float",
            py_fn!(py, kth_float_py(xs: PyObject, k: usize)),
        )?;
        m.add(
            py,
            "kth_element_int",
            py_fn!(py, kth_int_py(xs: PyObject, k: usize)),
        )?;
        m.add(
            py,
            "kth_element_uint",
            py_fn!(py, kth_uint_py(xs: PyObject, k: usize)),
        )?;

        Ok(())
    }
);

#[inline]
fn pylist_to_vec<T>(py: Python<'_>, xs: PyObject) -> PyResult<Vec<T>>
where
    for<'a> T: FromPyObject<'a>,
{
    Vec::extract(py, &xs)
}

#[inline]
fn extract_ordered_floats<'a>(py: Python<'_>, obj: &'a PyObject) -> PyResult<Vec<OrderedFloat<f64>>> {
    let list = try!(obj.cast_as::<PyList>(py));

    let len = list.len(py);
    let mut v = Vec::with_capacity(len);

    for i in 0..len {
        let item = list.get_item(py, i);
        v.push(OrderedFloat(f64::extract(py, &item)?));
        item.release_ref(py);
    }

    Ok(v)
}

// Variance functions for float, int and uint
expander!(variance,
         (variance_f64_py, f64));


// Average functions for float, int and uint
expander!(mean,
         (mean_f64_py, f64), (mean_i64_py, i64), (mean_u64_py, u64));

// Harmonic mean has a meaning for floats only
expander!(harmonic_mean, (harmonic_mean_f64_py, f64), (harmonic_mean_f32_py, f32));

// Median, median_low and median_high
expander_mut!(median,
             (median_f64_py, f64), (median_i64_py, i64), (median_u64_py, u64));

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

expander_mut!(median_low, (median_low_i64_py, i64), (median_low_u64_py, u64));

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

expander_mut!(median_high, (median_high_i64_py, i64), (median_high_u64_py, u64));

// Mode for float, int, uint and str

// floats have to be converted to OrderedFloats explicitly,
// therefor can't be expanded with macros
fn mode_float_py(py: Python<'_>, xs: PyObject) -> PyResult<f64> {
    let ys = extract_ordered_floats(py, &xs)?;
    let res = match stat_funcs::mode::<OrderedFloat<f64>>(ys) {
        Ok(res) => Ok(res.into()),
        Err(err) => Err(err),
    };
    to_python_result(py, res)
}

expander!(mode,
         (mode_str_py, String), (mode_i64_py, i64), (mode_u64_py, u64));

// k-th order statistic for float, int and uint

// TODO: How to fold these guys?
fn kth_float_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<f64> {
    let mut ys = pylist_to_vec::<f64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_int_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<i64> {
    let mut ys = pylist_to_vec::<i64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

fn kth_uint_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<u64> {
    let mut ys = pylist_to_vec::<u64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}
