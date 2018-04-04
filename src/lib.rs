#![feature(proc_macro, specialization)]

extern crate pyo3;

use pyo3::{py, PyResult, Python, PyModule, PyList, ObjectProtocol, FromPyObject};
use std::ops::Add;

mod stat_funcs;


#[inline]
fn pylist_to_vec<'a, T: FromPyObject<'a>>(xs: &'a PyList) -> Vec<T> {

    let ys: Vec<T> = xs
        .iter()
        .map(|x| x.extract::<T>()
        .unwrap())
        .collect();

    ys

}

#[inline]
fn sum_pylist_elems<'a, T: FromPyObject<'a> + Add<Output=T> >(xs: &'a PyList, init: T) -> T {
    let net = xs.iter().fold(init, |acc, x|
        { acc + x.extract::<T>().unwrap() });

    net
}

#[py::modinit(libfast_stat)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "mean_float")]
    fn mean_float_py(xs: &PyList) -> PyResult<f64> {

        /// It is faster to implement mean straight-forward way but
        /// it is still slower than naive python implementation like
        /// sum(xs) / len(xs)

        let net = sum_pylist_elems(xs, 0.0);
        Ok(net / xs.len() as f64)

    }

    #[pyfn(m, "mean_int")]
    fn mean_int_py(xs: &PyList) -> PyResult<i64> {

        /// It is faster to implement mean straight-forward way but
        /// it is still slower than naive python implementation like
        /// sum(xs) / len(xs)

        let net = sum_pylist_elems(xs, 0);
        Ok(net / xs.len() as i64)

    }

    #[pyfn(m, "kth_float")]
    fn kth_float_py(xs: &PyList) -> PyResult<f64> {
        stat_funcs::kth_stat(&mut pylist_to_vec::<f64>(xs));
        Ok(1.0)
    }

    Ok(())
}
