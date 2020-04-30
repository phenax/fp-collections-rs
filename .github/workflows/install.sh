#!/bin/bash

sh <(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs) -y && \
cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy;
