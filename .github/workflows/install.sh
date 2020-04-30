#!/usr/bin/env bash

rustup update stable;

rustup toolchain install nightly && \
rustup component add clippy --toolchain=nightly;

cargo install cargo-cmd;

