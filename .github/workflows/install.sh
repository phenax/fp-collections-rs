#!/usr/bin/env bash

rustup update stable;

rustup toolchain install nightly && \
rustup component add clippy --toolchain=nightly;

cargo install cargo-cmd;


sudo apt-get install libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev libiberty-dev zlib1g-dev;

wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz && \
tar xzf master.tar.gz && \
cd kcov-master && \
mkdir build && \
cd build && \
cmake .. && \
make && \
sudo make install;
