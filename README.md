# Fast-statistics

Fast-statistics is a small package of various statistical methods for Python 2/Python 3 implemented in Rust. The idea was taken from Python 3 statistics package (see https://docs.python.org/3/library/statistics.html). This package is currently written on pure python and may work too slow on big data sets. Fast-statistics implements all the functionality as the original library has but it also works faster in most cases and can be compiled for both Python 2 and Python 3 with ease.

#### Why not use pandas or scipy?

Short answer - these sdks are great, though they are pretty heavy and complex as well. Sometimes you just need few simple ...

Usually, there are few variants of each function exist, one for floating point numbers, one for unsigned integers and one for signed integers.

Let's suppose we want to find a median of a given list of floating point numbers, we could simply write:
```python
from fast_statistics import median_float

median = median_float([2.0, 1.0, 3.0, 5.0, 7.0])
print (median)
```

Looks simple enough!

Another example, ```mode``` function also supports arrays of strings, so it is possible to write:
```python
from fast_statistics import mode_str

mode_element = mode_str(['aa', 'bb', 'cc', 'aa'])
print (mode_element)
```

Some functions support arrays of floating point numbers only, ```harmonic_mean``` could be a good example of such a function, see docs for more information.

### Important note
TODO: Add note about precision and bugs.

Pull-requests are welcome!
