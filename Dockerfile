# docker image for testing and benchmarking
FROM ubuntu:18.04 

# install required packages
RUN apt-get update && apt-get -y install curl && curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
RUN apt-get -y install python3 && apt-get -y install python3-pip && pip3 install setuptools_rust hypothesis
RUN apt-get -y install python

# prepare environment
RUN mkdir /root/fast_stat
WORKDIR /root/fast_stat
ENV PATH $PATH:/root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin

# cache requirements to speed up contaier update
COPY Cargo.lock Cargo.toml ./
RUN mkdir src
RUN echo "fn main() { () }" > src/lib.rs
RUN cargo build --release
RUN rm -rf ./src

# copy current folder into image and prepare for install
COPY . /root/fast_stat

# install lib
RUN python3 setup.py install

