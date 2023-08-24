#!/bin/bash

cargo build --target aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

cargo build --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu

[[ ! -d "bin" ]] && mkdir -p bin

cp target/aarch64-unknown-linux-gnu/debug/qsc bin/qsc+debug.aarch64
cp target/aarch64-unknown-linux-gnu/release/qsc bin/qsc.aarch64
cp target/x86_64-unknown-linux-gnu/debug/qsc bin/qsc+debug.x86_64
cp target/x86_64-unknown-linux-gnu/release/qsc bin/qsc.x86_64
