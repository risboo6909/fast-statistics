#!/bin/bash
set -ex

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2018-10-01 -y
export PATH="$HOME/.cargo/bin:$PATH"

cd /io

for PYBIN in /opt/python/{cp35-cp35m,cp36-cp36m,cp37-cp37m}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"
    "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
    "${PYBIN}/python" setup.py bdist_wheel
done

/opt/_internal/cpython-3.6.6/bin/python -m pip install wheel==0.31.1 || true
/opt/_internal/cpython-3.6.7/bin/python -m pip install wheel==0.31.1 || true

for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done

for PYBIN in /opt/python/{cp35-cp35m,cp36-cp36m,cp37-cp37m}/bin/; do
    "${PYBIN}/pip" install hypothesis
    "${PYBIN}/pip" install fast_statistics --no-index -f /io/dist
    "${PYBIN}/python" /io/tests/tests.py
done

