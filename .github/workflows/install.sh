#!/bin/bash

sh <(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs) -y && \
sh <(curl https://sh.rustup.rs -sSf) && \
cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy;
