#!/bin/bash

[[ -d "bin" ]] && rm -rf bin

mkdir bin

docker run -d \
    --name qsc-builder \
    -v "$(pwd):/usr/src/qsc" \
    -v "$HOME/.cargo:/root/.cargo" \
    -e "GITHUB_SHA=$GITHUB_SHA" \
    ghcr.io/redstonewizard08/quickscript/builder:latest \
    sleep infinity

# ================= Build =================

docker exec qsc-builder cargo zigbuild --target aarch64-unknown-linux-gnu
docker exec qsc-builder cargo zigbuild --target aarch64-unknown-linux-musl
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabi
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-musleabi
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabihf
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-musleabihf
docker exec qsc-builder cargo zigbuild --target x86_64-unknown-linux-gnu
docker exec qsc-builder cargo zigbuild --target x86_64-unknown-linux-musl
docker exec qsc-builder cargo zigbuild --target i686-unknown-linux-gnu
# docker exec qsc-builder cargo zigbuild --target i686-unknown-linux-musl

docker exec qsc-builder cargo zigbuild --target aarch64-unknown-linux-gnu --release
docker exec qsc-builder cargo zigbuild --target aarch64-unknown-linux-musl --release
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabi --release
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-musleabi --release
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabihf --release
# docker exec qsc-builder cargo zigbuild --target arm-unknown-linux-musleabihf --release
docker exec qsc-builder cargo zigbuild --target x86_64-unknown-linux-gnu --release
docker exec qsc-builder cargo zigbuild --target x86_64-unknown-linux-musl --release
docker exec qsc-builder cargo zigbuild --target i686-unknown-linux-gnu --release
# docker exec qsc-builder cargo zigbuild --target i686-unknown-linux-musl --release

# ================= Copy Artifacts =================

# Compiler

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

# Language Server

docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-gnu/debug/qsc-lsp bin/qsc-lsp-aarch64-gnu-debug
docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-musl/debug/qsc-lsp bin/qsc-lsp-aarch64-musl-debug
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-gnu/debug/qsc-lsp bin/qsc-lsp-x86_64-gnu-debug
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-musl/debug/qsc-lsp bin/qsc-lsp-x86_64-musl-debug
docker cp qsc-builder:/usr/src/qsc/target/i686-unknown-linux-gnu/debug/qsc-lsp bin/qsc-lsp-i686-gnu-debug

docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-gnu/release/qsc-lsp bin/qsc-lsp-aarch64-gnu-release
docker cp qsc-builder:/usr/src/qsc/target/aarch64-unknown-linux-musl/release/qsc-lsp bin/qsc-lsp-aarch64-musl-release
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-gnu/release/qsc-lsp bin/qsc-lsp-x86_64-gnu-release
docker cp qsc-builder:/usr/src/qsc/target/x86_64-unknown-linux-musl/release/qsc-lsp bin/qsc-lsp-x86_64-musl-release
docker cp qsc-builder:/usr/src/qsc/target/i686-unknown-linux-gnu/release/qsc-lsp bin/qsc-lsp-i686-gnu-release

# ================= Clean up =================

docker stop qsc-builder
docker rm qsc-builder
