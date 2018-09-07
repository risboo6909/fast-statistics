use num::Float;

use cpython::{FromPyObject, PyDrop, PyList, PyObject, PyResult, Python};
use ordered_float::OrderedFloat;

#[inline]
crate fn pylist_to_vec<T>(py: Python<'_>, xs: PyObject) -> PyResult<Vec<T>>
where
    for<'a> T: FromPyObject<'a>,
{
    Vec::extract(py, &xs)
}

#[inline]
crate fn extract_ordered_floats<T>(py: Python<'_>, obj: &PyObject) -> PyResult<Vec<OrderedFloat<T>>>
where
    for<'a> T: Float + FromPyObject<'a>,
{
    let list = obj.cast_as::<PyList>(py)?;

    let len = list.len(py);
    let mut v = Vec::with_capacity(len);

    for i in 0..len {
        let item = list.get_item(py, i);
        v.push(OrderedFloat(T::extract(py, &item)?));
        item.release_ref(py);
    }

    Ok(v)
}
