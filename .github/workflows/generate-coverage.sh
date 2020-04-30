#!/usr/bin/env bash

bin_dir=target/debug;
cov_dir=target/cov;
kcov=$(which kcov || echo "kcov-master/build/src/kcov");

root=$(pwd);

install_kcov() {
  sudo apt-get install libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev libiberty-dev zlib1g-dev;

  wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz && \
    tar xzf master.tar.gz && \
    cd kcov-master && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make;
}

report_cov() {
  ./$kcov --exclude-pattern=/.cargo,/usr/lib --verify $cov_dir "$1";
}

get_binaries() {
  find $bin_dir -maxdepth 1 -perm -111 -type f;
}

# Install kcov if required
which kcov || install_kcov;
cd $root;

# Build report
get_binaries | while read bin; do report_cov $bin; done;

