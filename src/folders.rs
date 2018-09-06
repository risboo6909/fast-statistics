macro_rules! fold_args {

    ($rust_func_name:ident, $( ($func_name:ident, [$($arg:ident:: $arg_type:ty),+] => $ret_type:ty) ),+) => {
        $(
            crate fn $func_name(py: Python<'_>, xs: PyObject, $($arg: $arg_type)+) -> PyResult<$ret_type> {
                let ys = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(ys, $($arg)+))
            }
        )+
    };


    (mut $rust_func_name:ident, $( ($func_name:ident, [$($arg:ident:: $arg_type:ty),+] => $ret_type:ty) ),+) => {
        $(
            crate fn $func_name(py: Python<'_>, xs: PyObject, $($arg: $arg_type)+) -> PyResult<$ret_type> {
                let mut ys = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(&mut ys, $($arg)+))
            }
        )+
    };


    (mut $rust_func_name:ident, $( ($func_name:ident => $ret_type:ty) ),+) => {
        $(
            crate fn $func_name(py: Python<'_>, xs: PyObject) -> PyResult<$ret_type> {
                let mut ys = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(&mut ys))
            }
        )+
    };


    ($rust_func_name:ident, $( ($func_name:ident => $ret_type:ty) ),+) => {
        $(
            crate fn $func_name(py: Python<'_>, xs: PyObject) -> PyResult<$ret_type> {
                let ys = pylist_to_vec(py, xs)?;
                to_python_result(py, stat_funcs::$rust_func_name(ys))
            }
        )+
    };

    (ord $rust_func_name:ident, $( ($func_name:ident, $ret_type:ty) ),+) => {
        $(
            crate fn $func_name(py: Python<'_>, xs: PyObject) -> PyResult<$ret_type> {
                let ys = extract_ordered_floats(py, &xs)?;
                let res = stat_funcs::$rust_func_name::<OrderedFloat<$ret_type>>(ys);
                to_python_result(py, res.map(|x| x.into()))
            }
        )+
    };

    (ord mut $rust_func_name:ident, $( ($func_name:ident, $ret_type:ty) ),+) => {
        $(
            crate fn $func_name(py: Python<'_>, xs: PyObject) -> PyResult<$ret_type> {
                let mut ys = extract_ordered_floats(py, &xs)?;
                let res = stat_funcs::$rust_func_name::<OrderedFloat<$ret_type>>(&mut ys);
                to_python_result(py, res.map(|x| x.into()))
            }
        )+
    };

}
