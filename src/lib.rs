#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate cpython;
#[macro_use]
mod folders;

mod stat_funcs;
mod utils;

use cpython::{PyObject, PyResult, Python};
use crate::stat_funcs::errors::to_python_result;
use crate::utils::{extract_ordered_floats, pylist_to_vec};
use ordered_float::OrderedFloat;

py_module_initializer!(
    libfast_stat,
    initlibfast_stat,
    PyInit_libfast_stat,
    |py, m| {
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


fold_args!(variance, (variance_f64_py, [] => f64), (variance_f32_py, [] => f32));

fold_args!(mean, (mean_f64_py, [] => f64), (mean_f32_py, [] => f32));

fold_args!(harmonic_mean, (harmonic_mean_f64_py, [] => f64), (harmonic_mean_f32_py, [] => f32));

fold_args!(mut median, (median_f64_py, [] => f64), (median_f32_py, [] => i64));

fold_args!(mode,
          (mode_str_py, [] => String), (mode_i64_py, [] => i64), (mode_i32_py, [] => i32),
          (mode_u64_py, [] => u64), (mode_u32_py, [] => u32));

fold_args!(mut kth_stat, (kth_elem_f64_py, [k::usize] => f64), (kth_elem_f32_py, [k::usize] => f32),
                         (kth_elem_u64_py, [k::usize] => u64), (kth_elem_u32_py, [k::usize] => u32),
                         (kth_elem_i64_py, [k::usize] => i64), (kth_elem_i32_py, [k::usize] => i32)
          );

fold_args!(ord mut median_low, (median_low_f32_py, f32), (median_low_f64_py, f64));
fold_args!(ord mut median_high, (median_high_f32_py, f32), (median_high_f64_py, f64));

fold_args!(ord mode, (mode_f32_py, f32), (mode_f64_py, f64));
