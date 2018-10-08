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
    m.add(py, "mean_f64", py_fn!(py, mean_f64_py(xs: PyObject)))?;
    m.add(py, "mean_f32", py_fn!(py, mean_f32_py(xs: PyObject)))?;
    m.add(
        py,
        "variance_f64",
        py_fn!(py, variance_f64_py(xs: PyObject)),
    )?;
    m.add(
        py,
        "variance_f32",
        py_fn!(py, variance_f32_py(xs: PyObject)),
    )?;
    m.add(py, "stdev_f64", py_fn!(py, stdev_f64_py(xs: PyObject)))?;
    m.add(py, "stdev_f32", py_fn!(py, stdev_f32_py(xs: PyObject)))?;
    m.add(
        py,
        "pvariance_f64",
        py_fn!(py, pvariance_f64_py(xs: PyObject)),
    )?;
    m.add(
        py,
        "pvariance_f32",
        py_fn!(py, pvariance_f32_py(xs: PyObject)),
    )?;
    m.add(py, "pstdev_f64", py_fn!(py, pstdev_f64_py(xs: PyObject)))?;
    m.add(py, "pstdev_f32", py_fn!(py, pstdev_f32_py(xs: PyObject)))?;
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

    m.add(py, "median_f64", py_fn!(py, median_f64_py(xs: PyObject)))?;
    m.add(py, "median_f32", py_fn!(py, median_f32_py(xs: PyObject)))?;

    m.add(
        py,
        "median_grouped_f64",
        py_fn!(py, median_grouped_f64_py(xs: PyObject, interval: usize)),
    )?;
    m.add(
        py,
        "median_grouped_f32",
        py_fn!(py, median_grouped_f32_py(xs: PyObject, interval: usize)),
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
});

gen_wrapper!(variance, (variance_f64_py, [] => f64), (variance_f32_py, [] => f32));

gen_wrapper!(pvariance, (pvariance_f64_py, [] => f64), (pvariance_f32_py, [] => f32));

gen_wrapper!(pstdev, (pstdev_f64_py, [] => f64), (pstdev_f32_py, [] => f32));

gen_wrapper!(stdev, (stdev_f64_py, [] => f64), (stdev_f32_py, [] => f32));

gen_wrapper!(mean, (mean_f64_py, [] => f64), (mean_f32_py, [] => f32));

gen_wrapper!(harmonic_mean, (harmonic_mean_f64_py, [] => f64), (harmonic_mean_f32_py, [] => f32));

gen_wrapper!(mut median, (median_f64_py, [] => f64), (median_f32_py, [] => f32));

gen_wrapper!(mode,
            (mode_str_py, [] => String), (mode_i64_py, [] => i64), (mode_i32_py, [] => i32),
            (mode_u64_py, [] => u64), (mode_u32_py, [] => u32));

gen_wrapper!(mut kth_stat, (kth_elem_f64_py, [k::usize] => f64),
                           (kth_elem_f32_py, [k::usize] => f32),
                           (kth_elem_u64_py, [k::usize] => u64),
                           (kth_elem_u32_py, [k::usize] => u32),
                           (kth_elem_i64_py, [k::usize] => i64),
                           (kth_elem_i32_py, [k::usize] => i32)
            );

gen_wrapper!(mut median_grouped, (median_grouped_f64_py, [interval::usize] => f64),
                                 (median_grouped_f32_py, [interval::usize] => f32)
            );

gen_wrapper!(ord mut median_low, (median_low_f32_py, [] => f32), (median_low_f64_py, [] => f64));
gen_wrapper!(ord mut median_high, (median_high_f32_py, [] => f32), (median_high_f64_py, [] => f64));

gen_wrapper!(ord mode, (mode_f32_py, [] => f32), (mode_f64_py, [] => f64));
