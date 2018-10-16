import time
import timeit
import random
import fast_stat
import statistics

from contextlib import contextmanager


result = '{}, time: {}'.format

@contextmanager
def time_it(descr):
    st = time.time()
    yield
    print('Benchmark {}, time taken: {}'.format(descr, time.time() - st))


# see https://stackoverflow.com/questions/8220801/how-to-use-timeit-module for details how to properly use timeit

def bench_mode_faststat(length, repeat):
    xs = [0 if random.randint(0, 10) < 8 else x for x in range(length)]
    print('\nbenchmarking mode against list of ints of length {}, repeats {}'.format(length, repeat))
    print(result('fast_stat.mode_i64', min(timeit.repeat(lambda: fast_stat.mode_i64(xs), number=repeat))))
    print(result('fast_stat.mode_i32', timeit.timeit(lambda: fast_stat.mode_i32(xs), number=repeat)))
    print(result('fast_stat.mode_u64', timeit.timeit(lambda: fast_stat.mode_u64(xs), number=repeat)))
    print(result('fast_stat.mode_u32', timeit.timeit(lambda: fast_stat.mode_u32(xs), number=repeat)))

    xs = [0.0 if random.randint(0, 10) < 8 else random.uniform(0.1, 1.0) for x in range(length)]
    print('\nbenchmarking mode against list of floats of length {}, repeats {}'.format(length, repeat))
    print(result('fast_stat.mode_f64', timeit.timeit(lambda: fast_stat.mode_f64(xs), number=repeat)))
    print(result('fast_stat.mode_f32',timeit.timeit(lambda: fast_stat.mode_f32(xs), number=repeat)))


def bench_mode_stat(length, repeat):
    xs = [0 if random.randint(0, 10) < 8 else x for x in range(length)]
    print('\nbenchmarking mode against list of ints of length {}, repeats {}'.format(length, repeat))
    print(result('statistics.mode', timeit.timeit(lambda: statistics.mode(xs), number=repeat)))

    xs = [0.0 if random.randint(0, 10) < 8 else random.uniform(0.1, 1.0) for x in range(length)]
    print('\nbenchmarking mode against list of floats of length {}, repeats {}'.format(length, repeat))
    print(result('statistics.mode', timeit.timeit(lambda: statistics.mode(xs), number=repeat)))


if __name__ == '__main__':
    print('\nrust implementation')
    bench_mode_faststat(length=1000000, repeat=100)
    print('\npure python implementation')
    bench_mode_stat(length=1000000, repeat=100)

