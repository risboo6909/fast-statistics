use cpython::{PyResult, Python, PyErr};


py_exception!(fast_stat, StatisticsError);


#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "Division by zero")]
    ZeroDivisionError,
    #[fail(display = "Wrong type")]
    WrongTypeError,
}


#[inline]
pub fn to_python_result<T>(py: Python, res: Result<T, MyError>) -> PyResult<T> {
    match res {
        Err(MyError::ZeroDivisionError) => Err(PyErr::new::<StatisticsError, _>
            (py, "Integer division or modulo by zero")),
        Err(MyError::WrongTypeError) => Err(PyErr::new::<StatisticsError, _>
            (py, "Unsupported items type")),
        Ok(x) => Ok(x)
    }
}
