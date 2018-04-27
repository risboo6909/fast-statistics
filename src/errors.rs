use cpython::{PyResult, Python, PyErr, exc};


#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "Division by zero")]
    DivisionByZero,
}


#[inline]
pub fn to_python_result<T>(py: Python, res: Result<T, MyError>) -> PyResult<T> {
    match res {
        Err(MyError::DivisionByZero) => Err(PyErr::new::<exc::ZeroDivisionError, _>
            (py, "integer division or modulo by zero")),
        Ok(x) => Ok(x)
    }
}
