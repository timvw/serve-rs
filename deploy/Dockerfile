FROM rust:1.57 as builder
RUN apt-get update
RUN apt-get install -y cmake musl-tools clang libc++-dev build-essential autoconf libtool pkg-config

FROM builder AS build
WORKDIR /opt/app

# create an empty project
RUN mkdir -p ./src/bin && \
    echo "fn main() {}" > ./src/bin/test.rs

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release

# copy your source tree
COPY ./build.rs ./build.rs
COPY ./proto ./proto
COPY ./src ./src

# build for release
RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:buster-slim

WORKDIR /opt/app
COPY --from=build /opt/app/target/release/broker /opt/app/broker

EXPOSE 50051
#ENV ROCKET_ADDRESS=0.0.0.0
ENTRYPOINT [ "/opt/app/broker" ]