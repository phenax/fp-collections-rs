#!/usr/bin/env bash

bin_dir=target/debug;
cov_dir=target/cov;

report_cov() {
  kcov --exclude-pattern=/.cargo,/usr/lib --verify $cov_dir "$1";
}

get_binaries() {
  find $bin_dir -maxdepth 1 -perm -111 -type f;
}

get_binaries | while read bin; do report_cov $bin; done;

