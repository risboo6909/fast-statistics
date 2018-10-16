#![warn(rust_2018_idioms)]
#![feature(crate_visibility_modifier)]

#[macro_use]
mod gen_macro;

mod stat_funcs;
mod utils;

use cpython::*;
use crate::stat_funcs::errors::to_python_result;
use crate::utils::{extract_ordered_floats, pylist_to_vec};
use ordered_float::OrderedFloat;

py_module_initializer!(fast_stat, initfast_stat, PyInit_fast_stat, |py, m| {
    m.add(py, "mean", py_fn!(py, mean_py(xs: PyObject)))?;

    m.add(
        py,
        "variance",
        py_fn!(py, variance_py(xs: PyObject)),
    )?;

    m.add(py, "stdev", py_fn!(py, stdev_py(xs: PyObject)))?;

    m.add(
        py,
        "pvariance",
        py_fn!(py, pvariance_py(xs: PyObject)),
    )?;

    m.add(py, "pstdev", py_fn!(py, pstdev_py(xs: PyObject)))?;

    m.add(
        py,
        "harmonic_mean",
        py_fn!(py, harmonic_mean_py(xs: PyObject)),
    )?;

    m.add(py, "median", py_fn!(py, median_py(xs: PyObject)))?;

    m.add(
        py,
        "median_grouped",
        py_fn!(py, median_grouped_py(xs: PyObject, interval: usize)),
    )?;

    m.add(
        py,
        "median_low",
        py_fn!(py, median_low_py(xs: PyObject)),
    )?;

    m.add(
        py,
        "median_high",
        py_fn!(py, median_high_py(xs: PyObject)),
    )?;

    m.add(py, "mode_float", py_fn!(py, mode_float_py(xs: PyObject)))?;
    m.add(py, "mode_int", py_fn!(py, mode_int_py(xs: PyObject)))?;
    m.add(py, "mode_uint", py_fn!(py, mode_uint_py(xs: PyObject)))?;
    m.add(py, "mode_str", py_fn!(py, mode_str_py(xs: PyObject)))?;

    m.add(
        py,
        "kth_elem_float",
        py_fn!(py, kth_elem_float_py(xs: PyObject, k: usize)),
    )?;

    m.add(
        py,
        "kth_elem_int",
        py_fn!(py, kth_elem_int_py(xs: PyObject, k: usize)),
    )?;

    m.add(
        py,
        "kth_elem_uint",
        py_fn!(py, kth_elem_uint_py(xs: PyObject, k: usize)),
    )?;

    Ok(())
});

gen_wrapper!(variance, (variance_py, [] => f64));

gen_wrapper!(pvariance, (pvariance_py, [] => f64));

gen_wrapper!(pstdev, (pstdev_py, [] => f64));

gen_wrapper!(stdev, (stdev_py, [] => f64));

gen_wrapper!(mean, (mean_py, [] => f64));

gen_wrapper!(harmonic_mean, (harmonic_mean_py, [] => f64));

gen_wrapper!(mut median, (median_py, [] => f64));

gen_wrapper!(mode,
            (mode_str_py, [] => String), (mode_int_py, [] => i64), (mode_uint_py, [] => u64));
gen_wrapper!(ord mode, (mode_float_py, [] => f64));


gen_wrapper!(mut kth_stat, (kth_elem_float_py, [k::usize] => f64),
                           (kth_elem_uint_py, [k::usize] => u64),
                           (kth_elem_int_py, [k::usize] => i64)
            );

gen_wrapper!(mut median_grouped, (median_grouped_py, [interval::usize] => f64));

gen_wrapper!(ord mut median_low, (median_low_py, [] => f64));
gen_wrapper!(ord mut median_high, (median_high_py, [] => f64));
