use cpython::{PyResult, Python, PyErr};


py_exception!(fast_stat, StatisticsError);


#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "Integer division or modulo by zero")]
    ZeroDivisionError,
    #[fail(display = "Wrong type")]
    WrongTypeError,
}


#[inline]
pub fn to_python_result<T>(py: Python, res: Result<T, MyError>) -> PyResult<T> {
    match res {
        Err(err) => Err(PyErr::new::<StatisticsError, _>(py, format!("{}", err))),
        Ok(x) => Ok(x)
    }
}
