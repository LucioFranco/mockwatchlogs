FROM rust:1.33.0 as build

WORKDIR /usr/src/mockwatchlogs

# Fetch external dependencies.
RUN mkdir -p src && touch src/lib.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

COPY src src
RUN cargo build -p mockwatchlogs --bin mockwatchlogs --frozen --release

FROM ubuntu:latest as runtime
WORKDIR /mockwatchlogs
COPY --from=build /usr/src/mockwatchlogs/target/release/mockwatchlogs ./mockwatchlogs
ENTRYPOINT ["/mockwatchlogs/mockwatchlogs"]
