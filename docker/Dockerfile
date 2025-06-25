# SPDX-License-Identifier: LGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

FROM rust:1.85.1-slim

RUN    dpkg --add-architecture armhf \
    && dpkg --add-architecture arm64 \
    && dpkg --add-architecture i386 \
    && apt-get update \
    && apt-get install -y \
        libiio-dev:armhf \
        libsensors4-dev:armhf \
        gcc-arm-linux-gnueabihf \
    && apt-get install -y \
        libiio-dev:arm64 \
        libsensors4-dev:arm64 \
        gcc-aarch64-linux-gnu \
    && apt-get install -y \
        libiio-dev \
        libsensors4-dev

# HACK: Installing 32-bit and 64-bit versions at the same time is not possible
# with apt. (one replaces the other) Therefore the CI must install the correct
# version beforehand. Installing both versions once simplifies installation in
# the CI since dependencies are already available.
RUN    apt-get download \
        python3.11-minimal \
        python3.11-minimal:i386 \
    && apt-get -y install \
        python3.11-minimal:i386 \
    && apt-get -y install \
        python3.11-minimal

RUN rustup target add \
        armv7-unknown-linux-gnueabihf \
        aarch64-unknown-linux-gnu \
    && rustup component add rustfmt clippy \
    && cargo install cbindgen@0.28.0 --locked \
    && cargo install cargo-deb@2.9.3 --locked
