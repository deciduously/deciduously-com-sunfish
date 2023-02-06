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
FROM rust:1.67.1
#RUN apt-get update && apt-get -y upgrade && apt-get install -y curl && apt-get -y autoremove && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . $HOME/.cargo/env
RUN mkdir -p /usr/src/deciduously-com
WORKDIR /usr/src/deciduously-com
COPY content/ ./content
COPY layouts/ ./layouts
COPY routes/ ./routes
COPY static/ ./static
COPY ui/ ./ui
COPY Cargo.toml main.rs build.rs serve.rs ./
RUN cargo build --release
CMD ["target/release/deciduously_com", "--host", "0.0.0.0", "-port", "8080"]
