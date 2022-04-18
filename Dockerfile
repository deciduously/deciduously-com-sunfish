FROM rust:1.60 AS builder
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir -p /usr/src/deciduously-com
WORKDIR /usr/src/deciduously-com

# Copy the source and build the application.
COPY Cargo.toml main.rs build.rs serve.rs ./
COPY content/ layouts/ routes/ static/ ui/ ./
RUN cargo build --release

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /usr/src/deciduously-com/target/x86_64-unknown-linux-musl/release/deciduously_com .
USER 1000
CMD ["./deciduously_com", "--host", "0.0.0.0", "-port", "8080"]
