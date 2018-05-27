# Fast-statistics

Fast-statistics is a small package of various statistical methods for Python 2/Python 3 implemented in Rust. The idea was taken from Python 3 statistics package (see https://docs.python.org/3/library/statistics.html). This package is currently written on pure python and may work too slow on big data sets. Fast-statistics implements all the functionality as the original library has but it also works faster in most cases and can be compiled for both Python 2 and Python 3 with ease.

Usually, there are few variants of each function exist, one for floating point numbers, one for unsigned integers and one for signed integers.

Let's suppose we want to find a median of a given list of floating point numbers, we could simply write:
```python
from fast_statistics import median_float

median = median_float([2.0, 1.0, 3.0, 5.0, 7.0])
print (median)
```

Looks simple enough!

### Important note
TODO: Add note about precision and bugs.

Pull-requests are welcome!
