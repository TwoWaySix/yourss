# Build Stage
FROM rust AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new deciduously-com
WORKDIR /usr/src/yourss
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
COPY templates ./templates
COPY static ./static
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/yourss .
USER 1000
CMD ["./yourss-fileserver", "-a", "0.0.0.0", "-p", "8080"]
