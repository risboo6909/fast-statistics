use cpython::{PyResult, Python, PyErr};


py_exception!(fast_stat, StatisticsError);


#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "Division by zero")]
    ZeroDivisionError,
}


#[inline]
pub fn to_python_result<T>(py: Python, res: Result<T, MyError>) -> PyResult<T> {
    match res {
        Err(MyError::ZeroDivisionError) => Err(PyErr::new::<StatisticsError, _>
            (py, "Integer division or modulo by zero")),
        Ok(x) => Ok(x)
    }
}
