#!/bin/bash

sh <(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs) -y && \
  rustup component add clippy --toolchain=nightly;
