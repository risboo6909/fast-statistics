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
    xs = data.draw(lists(floats(allow_nan=False, allow_infinity=False), min_size=2))
    interval = data.draw(integers(min_value=1, max_value=len(xs)-1))
    assert isclose(fast_stat.median_grouped(xs, interval), statistics.median_grouped(xs, interval)) is True


if __name__ == '__main__':
    test_with_msg('Testing median function', test_median)
    test_with_msg('Testing median_low function', test_median_low)
    test_with_msg('Testing median_high function', test_median_high)
    test_with_msg('Testing median_grouped function', test_median_grouped)

