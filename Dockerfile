FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/mc_watcher ./app

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/mc_watcher /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/mc_watcher"]
