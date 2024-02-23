#!/bin/bash

[[ -d "bin" ]] && rm -rf bin

mkdir bin

docker run -d \
    --name qsc-builder \
    -v "$(pwd)/target:/usr/src/qsc/target" \
    ghcr.io/redstonewizard08/quickscript/builder:latest \
    sleep infinity

docker exec -it qsc-builder cargo zigbuild --target aarch64-unknown-linux-gnu
docker exec -it qsc-builder cargo zigbuild --target aarch64-unknown-linux-musl
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabi
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-musleabi
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabihf
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-musleabihf
docker exec -it qsc-builder cargo zigbuild --target x86_64-unknown-linux-gnu
docker exec -it qsc-builder cargo zigbuild --target x86_64-unknown-linux-musl
docker exec -it qsc-builder cargo zigbuild --target i686-unknown-linux-gnu
# docker exec -it qsc-builder cargo zigbuild --target i686-unknown-linux-musl

docker exec -it qsc-builder cargo zigbuild --target aarch64-unknown-linux-gnu --release
docker exec -it qsc-builder cargo zigbuild --target aarch64-unknown-linux-musl --release
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabi --release
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-musleabi --release
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-gnueabihf --release
# docker exec -it qsc-builder cargo zigbuild --target arm-unknown-linux-musleabihf --release
docker exec -it qsc-builder cargo zigbuild --target x86_64-unknown-linux-gnu --release
docker exec -it qsc-builder cargo zigbuild --target x86_64-unknown-linux-musl --release
docker exec -it qsc-builder cargo zigbuild --target i686-unknown-linux-gnu --release
# docker exec -it qsc-builder cargo zigbuild --target i686-unknown-linux-musl --release

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

docker stop qsc-builder
docker rm qsc-builder
