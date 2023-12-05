# Use the official Rust image as a parent image
FROM rust:latest

# Install Chrome

RUN apt-get update && apt-get install -y cmake pkg-config libssl-dev openssl libopus-dev  youtube-dl ffmpeg

# Set the working directory in the container
WORKDIR /usr/src/rustycrab-api

# Copy the source code of rustycrab-api and rustycrab-model into the container
COPY ./rustycrab-api ./
COPY ./.env ./
COPY ./rustycrab-model /usr/src/rustycrab-model
COPY ./spotify-api /usr/src/spotify-api

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/rustycrab-api"]