#!/bin/sh

yes | sh -c "$(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs)" && \
yes | sh -c "$(curl https://sh.rustup.rs -sSf | sh)" && \
cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy;
