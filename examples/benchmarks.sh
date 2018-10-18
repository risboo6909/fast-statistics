#!/bin/bash

# based on replies from https://stackoverflow.com/questions/8220801/how-to-use-timeit-module

PYTHONPATH=../build/lib.linux-x86_64-3.6/


imports="
import random
import fast_stat
import statistics
"

dataset_len=1000000

printf "\nData set size is $dataset_len elements\n"

printf "\nMode computation benchmarks\n\n"

setup="
$imports
xs = [0 if random.randint(0, 10) < 8 else x for x in range($dataset_len)]
"
echo "fast_stat.mode_int" `python3 -m timeit -s "$setup" "fast_stat.mode_int(xs)"`
echo "fast_stat.mode_uint" `python3 -m timeit -s "$setup" "fast_stat.mode_uint(xs)"`
echo "statistics.mode" `python3 -m timeit -s "$setup" "statistics.mode(xs)"`

setup="
$imports
xs = [0.0 if random.randint(0, 10) < 8 else random.uniform(0.1, 1.0) for x in range($dataset_len)]
"
echo
echo "fast_stat.mode_float" `python3 -m timeit -s "$setup" "fast_stat.mode_float(xs)"`
echo "statistics.mode" `python3 -m timeit -s "$setup" "statistics.mode(xs)"`

printf "\n\nMedian search on heterogeneous data\n"

setup="
$imports
xs = [random.uniform(0, 1) for x in range($dataset_len)]
"
echo
echo "fast_stat.median" `python3 -m timeit -s "$setup" "fast_stat.median(xs)"`
echo "statistics.median" `python3 -m timeit -s "$setup" "statistics.median(xs)"`
echo "fast_stat.median_low" `python3 -m timeit -s "$setup" "fast_stat.median_low(xs)"`
echo "statistics.median_low" `python3 -m timeit -s "$setup" "statistics.median_low(xs)"`

setup="
$imports
xs = [random.uniform(0, 1.0) for x in range($dataset_len)]
"
echo
echo "fast_stat.median_grouped, interval 2" `python3 -m timeit -s "$setup" "fast_stat.median_grouped(xs, 2)"`
echo "statistics.median_grouped, interval 2" `python3 -m timeit -s "$setup" "statistics.median_grouped(xs, 2)"`
echo "fast_stat.median_grouped, interval 20" `python3 -m timeit -s "$setup" "fast_stat.median_grouped(xs, 20)"`
echo "statistics.median_grouped, interval 20" `python3 -m timeit -s "$setup" "statistics.median_grouped(xs, 20)"`

printf "\nMedian search on homogeneous data\n"

setup="
$imports
xs = [random.randint(0, 3) for x in range($dataset_len)]
"
echo
echo "fast_stat.median" `python3 -m timeit -s "$setup" "fast_stat.median(xs)"`
echo "statistics.median" `python3 -m timeit -s "$setup" "statistics.median(xs)"`
echo "fast_stat.median_high" `python3 -m timeit -s "$setup" "fast_stat.median_high(xs)"`
echo "statistics.median_high" `python3 -m timeit -s "$setup" "statistics.median_high(xs)"`

printf "\n\nHarmonic mean computation on random data\n"

setup="
$imports
xs = [random.uniform(0, 1.0) for x in range($dataset_len)]
"
echo
echo
echo "fast_stat.harmonic_mean" `python3 -m timeit -s "$setup" "fast_stat.harmonic_mean(xs)"`
echo "statistics.harmonic_mean" `python3 -m timeit -s "$setup" "statistics.harmonic_mean(xs)"`

printf "\n\nStandard deviation on random data\n"

setup="
$imports
xs = [random.uniform(0, 1.0) for x in range($dataset_len)]
"
echo
echo "fast_stat.stdev" `python3 -m timeit -s "$setup" "fast_stat.stdev(xs)"`
echo "statistics.stdev" `python3 -m timeit -s "$setup" "statistics.stdev(xs)"`

