# Fast-statistics

[![Build Status](https://travis-ci.com/risboo6909/fast-statistics.svg?token=sEoRH24ki1j8CFisEvo5&branch=master)](https://travis-ci.com/risboo6909/fast-statistics)

# Fast-statistics

Fast-statistics is a small package of various statistical methods for Python 3 (yet!, Python 2.7 is comming) implemented in Rust. The idea was taken from Python 3 statistics package (see https://docs.python.org/3/library/statistics.html). This package is written on pure python and may work too slow on big data sets. Fast-statistics implements same stat functions as the original library does but it also works faster in most cases.

#### When to use this library?

Short answer -  whenever you want to compute something with the better performance than default python implementation may provide.

Quick example.

Let's suppose we want to find a median of a given list of floating point numbers, we could simply write:
```python
from fast_statistics import median

median_value = median([2.0, 1.0, 3.0, 5.0, 7.0])
print (median_value)
```

compared to pure python version:

```python
from statistics import median

median_value = median([2.0, 1.0, 3.0, 5.0, 7.0])
print (median_value)
```

It looks as simple as a pure python version and although it can't be seen from this contrived example, works almost 10 times faster.

#### Limitations

Everythings has its price.

Major difference between python and rust implementation is that the latter one uses strict typing inside. This is actually a good thing, but at the same time it imposes some restrictions one should aware of.

1. Some functions work with real numbers only by default. Fast-statistics uses 
f64
 inside to represent real numbers. If you pass a list of integers to such a function, all its contents will be automatically converted into reals, so be careful using it because python doesn't introduce any limits to integers so conversion of very big numbers to 
f64
 may cause incorrect results. See the list of supported functions and their input and output type below.

2. Original python statistics package is able to work with arbitrary big integers, decimals and ratios. If you need one of those types then fast-statistics won't help you with that. It only works with native types such as 64 bit integers and floating point numbers.

3. There is a routine in python statistics package which is intended to improve accuracy of 
sum
 function by converting floating point numbers into fractions and then summing them up. 
sum
 is implicitly used in many various stat calculations. I didn't implement such a behaviour in the first version of fast-statistics, however this issue seems to matter in rare cases  when summing up very small and very large numbers at the same time. I think this feature will be implemented in further versions of the library.

#### Supported functions

### Important note
TODO: Add note about precision and bugs.

Pull-requests are welcome!
