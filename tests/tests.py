#!/usr/bin/env python3

'''Using hypothesis to ensure that library works consistently with python statistics package
'''

from hypothesis import given, reproduce_failure
from hypothesis.strategies import lists, floats, integers, data
from math import isclose
import fast_stat
import statistics


def test_with_msg(msg, test_func):
    print('{}... '.format(msg), end='')
    test_func()
    print('ok')


@given(data())
def test_kth_stat(data):
    xs = data.draw(lists(floats(allow_nan=False, allow_infinity=False), min_size=1))
    k = data.draw(integers(min_value=0, max_value=len(xs)-1))
    assert fast_stat.kth_elem_float(xs, k) == sorted(xs)[k]


@given(lists(floats(allow_nan=False, allow_infinity=False), min_size=1))
def test_median(xs):
    assert fast_stat.median(xs) == statistics.median(xs)


@given(lists(floats(allow_nan=False, allow_infinity=False), min_size=1))
def test_median_low(xs):
    assert isclose(fast_stat.median_low(xs), statistics.median_low(xs))


@given(lists(floats(allow_nan=False, allow_infinity=False), min_size=1))
def test_median_high(xs):
    assert isclose(fast_stat.median_high(xs), statistics.median_high(xs)) is True


@given(data())
def test_median_grouped(data):
    xs = data.draw(lists(floats(allow_nan=False, allow_infinity=False), min_size=1))
    interval = data.draw(integers(min_value=1, max_value=len(xs)))
    assert isclose(fast_stat.median_grouped(xs, interval), statistics.median_grouped(xs, interval)) is True


# width=32 is to prevent floating point OverflowError
@given(lists(floats(allow_nan=False, allow_infinity=False, width=32), min_size=2))
def test_stdev(xs):
    assert isclose(fast_stat.stdev(xs), statistics.stdev(xs)) is True


# width=32 is to prevent floating point OverflowError
@given(lists(floats(allow_nan=False, allow_infinity=False, width=32), min_size=2))
def test_pstdev(xs):
    assert isclose(fast_stat.pstdev(xs), statistics.pstdev(xs)) is True


# width=32 is to prevent floating point OverflowError
@given(lists(floats(allow_nan=False, allow_infinity=False, width=32), min_size=2))
def test_variance(xs):
    assert isclose(fast_stat.variance(xs), statistics.variance(xs)) is True


# width=32 is to prevent floating point OverflowError
@given(lists(floats(allow_nan=False, allow_infinity=False, width=32), min_size=2))
def test_pvariance(xs):
    assert isclose(fast_stat.pvariance(xs), statistics.pvariance(xs)) is True


# width=32 is to prevent floating point OverflowError
# TODO: check rust floats
@given(lists(floats(allow_nan=False, allow_infinity=False, min_value=0.0, width=32), min_size=1))
def test_harmonic_mean(xs):
    assert isclose(fast_stat.harmonic_mean(xs), statistics.harmonic_mean(xs)) is True


if __name__ == '__main__':
    test_with_msg('Testing kth_stat', test_kth_stat)
    test_with_msg('Testing median', test_median)
    test_with_msg('Testing median_low', test_median_low)
    test_with_msg('Testing median_high', test_median_high)
    test_with_msg('Testing median_grouped', test_median_grouped)
    test_with_msg('Testing stdev', test_stdev)
    test_with_msg('Testing pstdev', test_pstdev)
    test_with_msg('Testing variance', test_variance)
    test_with_msg('Testing pvariance', test_pvariance)
    test_with_msg('Testing harmonic_mean', test_harmonic_mean)

