# docker image for testing and benchmarking
FROM ubuntu:18.04 

# install required packages
RUN apt-get update && apt-get -y install curl && curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
RUN apt-get -y install python3 && apt-get -y install python3-pip && pip3 install setuptools_rust
RUN apt-get -y install python

# copy current folder into image and prepare for install
COPY . /root/fast_stat
ENV PATH $PATH:/root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin
WORKDIR /root/fast_stat

# install lib
RUN python3 setup.py install

