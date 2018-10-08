use num::Float;

use cpython::{FromPyObject, PyObject, PyResult, Python};
use ordered_float::{NotNaN, OrderedFloat};

#[inline]
crate fn pylist_to_vec<T>(py: Python<'_>, xs: &PyObject) -> PyResult<Vec<T>>
where
    for<'a> T: FromPyObject<'a>,
{
    Vec::extract(py, &xs)
}

#[inline]
crate fn into_mut_notnan<T>(xs: &mut [T]) -> &mut [NotNaN<T>]
where
    T: Float,
{
    unsafe { &mut *(xs as *mut [T] as *mut [ordered_float::NotNaN<T>]) }
}

#[inline]
crate fn extract_ordered_floats<T>(py: Python<'_>, xs: &PyObject) -> PyResult<Vec<OrderedFloat<T>>>
where
    for<'a> T: Float + FromPyObject<'a>,
{
    let tmp = &Vec::extract(py, &xs)?[..];
    let ys = unsafe { &*(tmp as *const [T] as *const [ordered_float::OrderedFloat<T>]) };

    Ok(ys.to_vec())
}

#[test]
fn test_extract_notnan() {
    let mut xs = vec![1.0, 2.5, 3.7];
    let ys = into_mut_notnan(&mut xs);

    assert_eq!(
        ys,
        [
            NotNaN::new(1.0).unwrap(),
            NotNaN::new(2.5).unwrap(),
            NotNaN::new(3.7).unwrap()
        ]
    );
}
