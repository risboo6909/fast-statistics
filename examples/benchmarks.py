from contextlib import contextmanager
import fast_stat


@contextmanager
def time_it(metric_name, log_time=True, tier=None):
    st = time.time()
    yield
    return time.time() - st


