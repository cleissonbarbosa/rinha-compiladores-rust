FROM rust:1.70-bullseye as build

WORKDIR /agent
ARG CARGO_FLAGS="--release"

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY examples examples

# To improve performance and prevent the entire registry from being downloaded
# see https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build ${CARGO_FLAGS}

FROM debian:bullseye-slim as agent

WORKDIR /agent

COPY --from=build /agent/target/release/main /agent/rinha
COPY --from=build /agent/examples /agent/examples

ENTRYPOINT ["/agent/rinha", "./examples/fib.rinha"]
