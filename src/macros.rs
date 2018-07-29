macro_rules! expander {
    ($rust_func_name:ident, $( ($f_name:ident, $t_name:ty) ),+) => {
        $(
            fn $f_name(py: Python<'_>, xs: PyObject) -> PyResult<$t_name> {
                let ys: Vec<$t_name> = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(ys))
            }
        )+
    }
}

macro_rules! expander_mut {
    ($rust_func_name:ident, $( ($f_name:ident, $t_name:ty) ),+) => {
        $(
            fn $f_name(py: Python<'_>, xs: PyObject) -> PyResult<$t_name> {
                let mut ys: Vec<$t_name> = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(&mut ys))
            }
        )+
    }
}
