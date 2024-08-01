ARG RUST_VERSION=1.80

ARG BUILDER_IMAGE="rust:${RUST_VERSION}-alpine"
ARG RUNNER_IMAGE="scratch"

FROM ${BUILDER_IMAGE} AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app

# Copy the source and build the application.
COPY Cargo.toml          \
	Cargo.lock             \
	main.rs                \
	build.rs               \
	serve.rs               \
	./

COPY .cargo .cargo
COPY content content
COPY layouts layouts
COPY routes routes
COPY static static
COPY ui ui

RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy the statically-linked binary into a scratch container.
FROM ${RUNNER_IMAGE}
WORKDIR "/app"
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/deciduously_com_sunfish ./
CMD ["/app/deciduously_com_sunfish", "--host", "0.0.0.0", "--port", "8080"]
