FROM rust:1.57.0 as builder
WORKDIR /app
RUN apt-get update -yqq \
    && apt-get install -yqq cmake g++ \
    && apt install -y tzdata
ENV TZ=Asia/Tokyo
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo install --path .
RUN rm -f target/debug/deps/boilerplate*
COPY ./src ./src
COPY ./.env ./.env
#COPY ./diesel.toml ./diesel.toml
RUN cargo build --release
CMD ["cargo", "run", "--release"]

FROM debian:buster-slim as target
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/boilerplate /usr/local/bin/boilerplate
CMD ["boilerplate"]
