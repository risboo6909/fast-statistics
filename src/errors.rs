use cpython::{PyResult, Python, PyErr};


py_exception!(fast_stat, StatisticsError);


#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "integer division or modulo by zero")]
    ZeroDivisionError,
    #[fail(display = "harmonic_mean requires at least one data point")]
    HarmonicNoDataPoints,
    #[fail(display = "no unique mode; found {} equally common values", modes)]
    NoUniqueMode {
        modes: usize,
    },
    #[fail(display = "no mode for empty data")]
    NoModeEmptyData,
    #[fail(display = "wrong type")]
    WrongTypeError,
}


#[inline]
pub fn to_python_result<T>(py: Python, res: Result<T, MyError>) -> PyResult<T> {
    match res {
        Err(err) => Err(PyErr::new::<StatisticsError, _>(py, format!("{}", err))),
        Ok(x) => Ok(x)
    }
}
