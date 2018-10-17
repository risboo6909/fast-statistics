#!/bin/bash

# based on replies from https://stackoverflow.com/questions/8220801/how-to-use-timeit-module

PYTHONPATH=../build/lib.linux-x86_64-3.6/


imports="
import random
import fast_stat
import statistics
"

printf "\nMode computation benchmarks\n\n"

setup="
$imports
xs = [0 if random.randint(0, 10) < 8 else x for x in range(100000)]
"
echo "fast_stat.mode_int" `python3 -m timeit -s "$setup" "fast_stat.mode_int(xs)"`
echo "fast_stat.mode_uint" `python3 -m timeit -s "$setup" "fast_stat.mode_uint(xs)"`
echo "statistics.mode" `python3 -m timeit -s "$setup" "statistics.mode(xs)"`

setup="
$imports
xs = [0.0 if random.randint(0, 10) < 8 else random.uniform(0.1, 1.0) for x in range(100000)]
"
echo
echo "fast_stat.mode_float" `python3 -m timeit -s "$setup" "fast_stat.mode_float(xs)"`
echo "statistics.mode" `python3 -m timeit -s "$setup" "statistics.mode(xs)"`

printf "\n\nMedian search on heterogeneous data\n"

setup="
$imports
xs = [random.uniform(0, 1) for x in range(1000000)]
"
echo
echo "fast_stat.median" `python3 -m timeit -s "$setup" "fast_stat.median(xs)"`
echo "statistics.median" `python3 -m timeit -s "$setup" "statistics.median(xs)"`
echo "fast_stat.median_low" `python3 -m timeit -s "$setup" "fast_stat.median_low(xs)"`
echo "statistics.median_low" `python3 -m timeit -s "$setup" "statistics.median_low(xs)"`

printf "\nMedian search on homogeneous data\n"

setup="
$imports
xs = [random.randint(0, 3) for x in range(1000000)]
"
echo
echo "fast_stat.median" `python3 -m timeit -s "$setup" "fast_stat.median(xs)"`
echo "statistics.median" `python3 -m timeit -s "$setup" "statistics.median(xs)"`
echo "fast_stat.median_high" `python3 -m timeit -s "$setup" "fast_stat.median_high(xs)"`
echo "statistics.median_high" `python3 -m timeit -s "$setup" "statistics.median_high(xs)"`

