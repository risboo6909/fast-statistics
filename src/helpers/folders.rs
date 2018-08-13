macro_rules! fold {
    ($rust_func_name:ident, $( ($f_name:ident, $t_name:ty) ),+) => {
        $(
            crate fn $f_name(py: Python<'_>, xs: PyObject) -> PyResult<$t_name> {
                let ys = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(ys))
            }
        )+
    }
}

macro_rules! fold_mut {
    ($rust_func_name:ident, $( ($f_name:ident, $t_name:ty) ),+) => {
        $(
            crate fn $f_name(py: Python<'_>, xs: PyObject) -> PyResult<$t_name> {
                let mut ys = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(&mut ys))
            }
        )+
    }
}

macro_rules! fold_ordered_float {
    ($rust_func_name:ident, $( ($f_name:ident, $t_name:ty) ),+) => {
        $(
            crate fn $f_name(py: Python<'_>, xs: PyObject) -> PyResult<$t_name> {
                let ys = extract_ordered_floats(py, &xs)?;
                let res = stat_funcs::$rust_func_name::<OrderedFloat<$t_name>>(ys);
                to_python_result(py, res.map(|x| x.into()))
            }
        )+
    }
}

macro_rules! fold_ordered_float_mut {
    ($rust_func_name:ident, $( ($f_name:ident, $t_name:ty) ),+) => {
        $(
            crate fn $f_name(py: Python<'_>, xs: PyObject) -> PyResult<$t_name> {
                let mut ys = extract_ordered_floats(py, &xs)?;
                let res = stat_funcs::$rust_func_name::<OrderedFloat<$t_name>>(&mut ys);
                to_python_result(py, res.map(|x| x.into()))
            }
        )+
    }
}
