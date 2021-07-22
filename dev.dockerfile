FROM rust:1.53.0

WORKDIR /app

# 0
RUN apt-get update -yqq && apt-get install -yqq cmake g++ && apt install -y tzdata
RUN cargo install diesel_cli --no-default-features --features postgres

# 1
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo install --path .
RUN rm -f target/debug/deps/boilerplate_actix-web_postgres*

# 2
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./.env ./.env
COPY ./diesel.toml ./diesel.toml

# 3
RUN cargo build --release

EXPOSE 8080

# 4
RUN apt-get install -y tzdata
ENV TZ=Asia/Tokyo

CMD ["cargo", "run", "--release"]

