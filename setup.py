#!/usr/bin/env python
import sys

from setuptools import setup
from setuptools_rust import Binding, RustExtension

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

setup(
    name="fast-statistics",
    version="0.1.0",
    author="Boris Tatarintsev",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
    ],
    description="Rust extension intended to be a faster version of built-in python statistics package",
    url="https://github.com/risboo6909/fast-statistics",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: POSIX",
    ],
    packages=["fast_stat"],
    rust_extensions=[RustExtension("fast_stat.fast_stat", binding=Binding.PyO3)],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)

