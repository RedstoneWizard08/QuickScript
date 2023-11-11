FROM rust:bookworm

ADD . /usr/src/qsc
WORKDIR /usr/src/qsc

ARG ZIG_VERSION=0.12.0-dev.1538+3f10b3ee1

RUN curl -fsSLo /zig.tar.xz \
        "https://ziglang.org/builds/zig-linux-$(uname -m)-${ZIG_VERSION}.tar.xz"

RUN mkdir -p /usr/share/zig
RUN tar -xJf /zig.tar.xz --strip-components 1 -C /usr/share/zig
RUN ln -sf /usr/share/zig/zig /usr/local/bin/zig

RUN echo "#!/bin/bash" > /usr/local/bin/zigcc && \
    echo '/usr/local/bin/zig cc "$@"' >> /usr/local/bin/zigcc && \
    chmod a+rx /usr/local/bin/zigcc

RUN cargo install cargo-zigbuild

# RUN apt-get update && \
#     apt-get -y install \
#         gcc-aarch64-linux-gnu \
#         gcc-arm-linux-gnueabi \
#         gcc-arm-linux-gnueabihf \
#         gcc-x86-64-linux-gnu \
#         gcc-i686-linux-gnu \
#         musl-dev

RUN rustup target add \
        aarch64-unknown-linux-musl \
        aarch64-unknown-linux-gnu \
        arm-unknown-linux-musleabi \
        arm-unknown-linux-gnueabi \
        arm-unknown-linux-musleabihf \
        arm-unknown-linux-gnueabihf \
        x86_64-unknown-linux-musl \
        x86_64-unknown-linux-gnu \
        i686-unknown-linux-musl \
        i686-unknown-linux-gnu

RUN cargo zigbuild --target aarch64-unknown-linux-gnu
RUN cargo zigbuild --target aarch64-unknown-linux-musl
# RUN cargo zigbuild --target arm-unknown-linux-gnueabi
# RUN cargo zigbuild --target arm-unknown-linux-musleabi
# RUN cargo zigbuild --target arm-unknown-linux-gnueabihf
# RUN cargo zigbuild --target arm-unknown-linux-musleabihf
RUN cargo zigbuild --target x86_64-unknown-linux-gnu
RUN cargo zigbuild --target x86_64-unknown-linux-musl
RUN cargo zigbuild --target i686-unknown-linux-gnu
# RUN cargo zigbuild --target i686-unknown-linux-musl

RUN cargo zigbuild --target aarch64-unknown-linux-gnu --release
RUN cargo zigbuild --target aarch64-unknown-linux-musl --release
# RUN cargo zigbuild --target arm-unknown-linux-gnueabi --release
# RUN cargo zigbuild --target arm-unknown-linux-musleabi --release
# RUN cargo zigbuild --target arm-unknown-linux-gnueabihf --release
# RUN cargo zigbuild --target arm-unknown-linux-musleabihf --release
RUN cargo zigbuild --target x86_64-unknown-linux-gnu --release
RUN cargo zigbuild --target x86_64-unknown-linux-musl --release
RUN cargo zigbuild --target i686-unknown-linux-gnu --release
# RUN cargo zigbuild --target i686-unknown-linux-musl --release
