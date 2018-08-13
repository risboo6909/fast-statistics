#[macro_use]
mod folders;

use ordered_float::OrderedFloat;
use num::{Num, FromPrimitive, Float};

use crate::stat_funcs;
use crate::stat_funcs::errors::to_python_result;

use cpython::{FromPyObject, PyDrop, PyList, PyObject, PyResult, Python};

#[inline]
fn pylist_to_vec<T>(py: Python<'_>, xs: PyObject) -> PyResult<Vec<T>>
    where for<'a> T: FromPyObject<'a> {
    Vec::extract(py, &xs)
}

#[inline]
fn extract_ordered_floats<T>(py: Python<'_>, obj: &PyObject) -> PyResult<Vec<OrderedFloat<T>>>
    where for<'a> T: Float + FromPyObject<'a> {

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

fold!(variance, (variance_f64_py, f64), (variance_f32_py, f32));

fold!(mean, (mean_f64_py, f64), (mean_f32_py, f32));

fold!(harmonic_mean, (harmonic_mean_f64_py, f64), (harmonic_mean_f32_py, f32));

fold_mut!(median, (median_f64_py, f64), (median_f32_py, i64));

fold_ordered_float_mut!(median_low, (median_low_f32_py, f32), (median_low_f64_py, f64));
fold_ordered_float_mut!(median_high, (median_high_f32_py, f32), (median_high_f64_py, f64));

fold_ordered_float!(mode, (mode_f32_py, f32), (mode_f64_py, f64));

fold!(mode,
         (mode_str_py, String), (mode_i64_py, i64), (mode_i32_py, i32), (mode_u64_py, u64),
         (mode_u32_py, u32));

// TODO: How to fold these guys?
crate fn kth_elem_f64_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<f64> {
    let mut ys = pylist_to_vec::<f64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

crate fn kth_elem_f32_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<f32> {
    let mut ys = pylist_to_vec::<f32>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

crate fn kth_elem_u64_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<u64> {
    let mut ys = pylist_to_vec::<u64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

crate fn kth_elem_u32_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<u32> {
    let mut ys = pylist_to_vec::<u32>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

crate fn kth_elem_i64_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<i64> {
    let mut ys = pylist_to_vec::<i64>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}

crate fn kth_elem_i32_py(py: Python<'_>, xs: PyObject, k: usize) -> PyResult<i32> {
    let mut ys = pylist_to_vec::<i32>(py, xs)?;
    to_python_result(py, stat_funcs::kth_stat(&mut ys, k))
}
