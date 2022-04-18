# FROM rust:1.60 AS builder
# RUN rustup target add x86_64-unknown-linux-musl
# RUN mkdir -p /usr/src/deciduously-com
# WORKDIR /usr/src/deciduously-com

# # Copy the source and build the application.
# COPY Cargo.toml main.rs build.rs serve.rs ./
# COPY .cargo/ content/ layouts/ routes/ static/ ui/ ./
# RUN cargo build --target x86_64-unknown-linux-musl

# # Copy the statically-linked binary into a scratch container.
# FROM scratch
# RUN mkdir -p /usr/local/bin
# WORKDIR /usr/local/bin
# COPY --from=builder /usr/src/deciduously-com/target/x86_64-unknown-linux-musl/debug/deciduously_com .
FROM ubuntu:20.04
COPY target/x86_64-unknown-linux-musl/debug/deciduously_com .
CMD ["deciduously_com", "--host", "0.0.0.0", "-port", "8080"]
