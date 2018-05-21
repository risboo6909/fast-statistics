#[macro_use] extern crate failure;
#[macro_use] extern crate cpython;
extern crate num;
extern crate ordered_float;

mod stat_funcs;
mod errors;

use std::clone::Clone;
use std::thread;
use cpython::{PyResult, Python, PyObject, PyList, PyDrop, FromPyObject};
use errors::{MyError, to_python_result};
use num::{Num, NumCast};
use ordered_float::*;


py_module_initializer!(libfast_stat, initlibfast_stat, PyInit_libfast_stat, |py, m| {
    m.add(py, "avg_float", py_fn!(py, avg_float_py(xs: PyObject)))?;
    m.add(py, "avg_int", py_fn!(py, avg_int_py(xs: PyObject)))?;
    m.add(py, "avg_uint", py_fn!(py, avg_uint_py(xs: PyObject)))?;

    m.add(py, "harmonic_mean", py_fn!(py, harmonic_mean_py(xs: PyObject)))?;

    m.add(py, "median_float", py_fn!(py, median_float_py(xs: PyObject)))?;
    m.add(py, "median_int", py_fn!(py, median_int_py(xs: PyObject)))?;
    m.add(py, "median_uint", py_fn!(py, median_uint_py(xs: PyObject)))?;

    m.add(py, "median_low_float", py_fn!(py, median_low_float_py(xs: PyObject)))?;
    m.add(py, "median_low_int", py_fn!(py, median_low_int_py(xs: PyObject)))?;
    m.add(py, "median_low_uint", py_fn!(py, median_low_uint_py(xs: PyObject)))?;

    m.add(py, "median_high_float", py_fn!(py, median_high_float_py(xs: PyObject)))?;
    m.add(py, "median_high_int", py_fn!(py, median_high_int_py(xs: PyObject)))?;
    m.add(py, "median_high_uint", py_fn!(py, median_high_uint_py(xs: PyObject)))?;

    m.add(py, "mode_float", py_fn!(py, mode_float_py(xs: PyObject)))?;

//    m.add(py, "kth_element", py_fn!(py, kth_py(xs: PyObject, k: usize)))?;

    m.add(py, "kth_element_float", py_fn!(py, kth_float_py(xs: PyObject, k: usize)))?;
    m.add(py, "kth_element_int", py_fn!(py, kth_int_py(xs: PyObject, k: usize)))?;
    m.add(py, "kth_element_uint", py_fn!(py, kth_uint_py(xs: PyObject, k: usize)))?;

    Ok(())
});


#[inline]
fn pylist_to_vec<T>(py: Python, xs: PyObject) -> PyResult<Vec<T>>
    where for<'a> T: FromPyObject<'a> {
    Vec::extract(py, &xs)
}

#[inline]
fn extract_floats<'a>(py: Python, obj: &'a PyObject) -> PyResult<Vec<OrderedFloat<f64>>> {
    let list = try!(obj.cast_as::<PyList>(py));

    let len = list.len(py);
    let mut v = Vec::with_capacity(len);

    for i in 0 .. len {
        let item = list.get_item(py, i);
        v.push(OrderedFloat(f64::extract(py, &item)?));
        item.release_ref(py);
    }

    Ok(v)
}

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

fn median_low_float_py(py: Python, xs: PyObject) -> PyResult<f64> {
    let mut ys: Vec<f64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median_low(&mut ys))
}

fn median_low_int_py(py: Python, xs: PyObject) -> PyResult<i64> {
    let mut ys: Vec<i64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median_low(&mut ys))
}

fn median_low_uint_py(py: Python, xs: PyObject) -> PyResult<u64> {
    let mut ys: Vec<u64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median_low(&mut ys))
}

fn median_high_float_py(py: Python, xs: PyObject) -> PyResult<f64> {
    let mut ys: Vec<f64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median_high(&mut ys))
}

fn median_high_int_py(py: Python, xs: PyObject) -> PyResult<i64> {
    let mut ys: Vec<i64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median_high(&mut ys))
}

fn median_high_uint_py(py: Python, xs: PyObject) -> PyResult<u64> {
    let mut ys: Vec<u64> = pylist_to_vec(py, xs)?;
    to_python_result(py, stat_funcs::median_high(&mut ys))
}


// mode for float, int, uint and str

fn mode_float_py(py: Python, xs: PyObject) -> PyResult<f64> {
    let mut ys = extract_floats(py, &xs)?;
    match stat_funcs::mode::<OrderedFloat<f64>>(ys) {
        Ok(res) => to_python_result(py, Ok(res.into())),
        Err(err) => to_python_result(py, Err(err))
    }
}


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

//fn kth_py<T>(py: Python, xs: PyObject, k: usize) -> PyResult<T> {
//
//    let t_str = to_python_result(py, detect_list_type(py, &xs))?;
//
////    match t_str.as_str() {
////        "int" => {
////            let mut ys = pylist_to_vec::<i32>(py, xs)?;
////            match stat_funcs::kth_stat::<i32>(&mut ys[..], k) {
////                Ok(x) => to_python_result(py, Box::new(Ok(NumCast::to_f64(&x).unwrap()))),
////                Err(err) => to_python_result(py, Box::new(Err(err)))
////            }
////        },
////        _ => {
////            let mut ys = pylist_to_vec::<f64>(py, xs)?;
////            match stat_funcs::kth_stat::<f64>(&mut ys[..], k) {
////                Ok(x) => to_python_result(py, Box::new(Ok(),
////                Err(err) => to_python_result(py, Box::new(Err(err)))
////            }
////        }
////    }
//
//    Ok()
//
//}
