# Boilerplate

## Setup

Clone

```shell
git clone {this-repository}
```

Install dependencies

```shell
cargo install --path .
```

Run application

```shell
# Normal
cargo run

# Run Release version
cargo run --release

# Auto Reloading
cargo watch -x 'run --bin boilerplate_actix-web_postgres'

# test: Test transaction to DB
cargo run -- test
```

Run docker

```shell
# dev environment
docker compose -f docker-compose.dev.yml -p dev up
docker compose -f docker-compose.dev.yml -p dev down

# test environment
```

Others

```shell
# format
cargo fmt
```
