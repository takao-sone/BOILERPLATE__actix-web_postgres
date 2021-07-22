# STEPS

## initial

Rustのバージョンアップグレード

```shell
rustup upgrade
```

```shell
cargo install cargo-watch
```

```shell
cargo add actix-web env_logger log dotenv
```

`dev.docerfile`を作成

```dockerfile
FROM rust:1.53.0

WORKDIR /app

# 0
RUN apt-get update -yqq \
    && apt-get install -yqq cmake g++ \
    && apt install -y tzdata
ENV TZ=Asia/Tokyo

# 1
#RUN cargo install diesel_cli --no-default-features --features postgres

# 2
COPY ./Cargo.toml ./Cargo.toml

# 3
#COPY ./migrations ./migrations
COPY ./src ./src
COPY ./.env ./.env
#COPY ./diesel.toml ./diesel.toml

# 4
RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]
```

`docker-compose.dev.yml`を作成

```dockerfile
version: '3'

services:

  # TODO: CLionのdebug機能を使いたい場合はコメントアウト
  boiler:
    container_name: boilerplate_actix-web_postgres
    build:
      context: .
      dockerfile: "dev.dockerfile"
    restart: always
    tty: true
    expose:
      - 8080
    ports:
      - 8080:8080
```

docker-composeファイルの実行方法
```shell
# run
docker compose -f docker-compose.dev.yml -p dev up  

# down
docker compose -f docker-compose.dev.yml -p dev down  
```

## Diesel

`deisel setup`以降は各アプリケーションで実行すること。

```toml
# Cargo.toml
diesel = { version = "1.4.7", features = ["postgres", "r2d2", "serde_json", "chrono", "uuidv07"] }
```

`src/lib.rs`を作成

```rust
// QueryIdなどのmacroを使うために必要
#[macro_use]
extern crate diesel;

extern crate dotenv;

mod api;
mod db;
```


## Redis

```shell
cargo add actix-redis time
```

## 