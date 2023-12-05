# Use the official Rust image as a parent image
FROM rust:latest

# RUN apt update &amp;&amp; apt upgrade -y 
# RUN apt install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross
 
# RUN rustup target add armv7-unknown-linux-gnueabihf 
# RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf 

RUN apt-get update && apt-get install -y cmake pkg-config libssl-dev openssl libopus-dev  youtube-dl ffmpeg


# Set the working directory in the container
WORKDIR /usr/src/rustycrab-api

# Copy the source code of rustycrab-api and rustycrab-model into the container
COPY ./rustycrab-api ./
COPY ./.env ./
COPY ./rustycrab-model /usr/src/rustycrab-model

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/rustycrab-api"]