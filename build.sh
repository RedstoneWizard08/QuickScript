#!/bin/bash

docker buildx build -t qsc-builder --load .

[[ -d "bin" ]] && rm -rf bin

mkdir bin

docker create --name qsc-builder qsc-builder

docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-gnu/debug/qsc bin/qsc-aarch64-gnu-debug
docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-musl/debug/qsc bin/qsc-aarch64-musl-debug
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-gnu/debug/qsc bin/qsc-x86_64-gnu-debug
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-musl/debug/qsc bin/qsc-x86_64-musl-debug
docker cp qsc-builder:/usr/src/qsc/target/i686-unknown-linux-gnu/debug/qsc bin/qsc-i686-gnu-debug

docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-gnu/release/qsc bin/qsc-aarch64-gnu-release
docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-musl/release/qsc bin/qsc-aarch64-musl-release
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-gnu/release/qsc bin/qsc-x86_64-gnu-release
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-musl/release/qsc bin/qsc-x86_64-musl-release
docker cp qsc-builder:/usr/src/qsc/target/i686-unknown-linux-gnu/release/qsc bin/qsc-i686-gnu-release

docker rm qsc-builder
