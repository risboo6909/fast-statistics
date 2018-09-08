use cpython::*;
use failure::Fail;

py_exception!(fast_stat, StatisticsError);

#[derive(Fail, Debug)]
crate enum MyError {
    #[fail(display = "harmonic_mean requires at least one data point")]
    HarmonicNoDataPoints,
    #[fail(display = "harmonic mean does not support negative values")]
    HarmonicNegatives,
    #[fail(
        display = "no unique mode; found {} equally common values",
        modes
    )]
    NoUniqueMode { modes: usize },
    #[fail(display = "no mode for empty data")]
    NoModeEmptyData,
    #[fail(display = "no median for empty data")]
    NoMedianEmptyData,
    #[fail(display = "variance requires at least two data points")]
    NoEnoughDataForVariance,
    #[fail(display = "population variance requires at least one data point")]
    NoEnoughDataForPopulationVariance,
    #[fail(display = "mean requires at least one data point")]
    NoEnoughDataForMean,
}

#[inline]
crate fn to_python_result<T>(py: Python<'_>, res: Result<T, MyError>) -> PyResult<T> {
    match res {
        Err(err) => Err(PyErr::new::<StatisticsError, _>(py, format!("{}", err))),
        Ok(x) => Ok(x),
    }
}
