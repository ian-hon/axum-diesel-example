# axum-diesel-example

## Prerequisites

### Install `diesel_cli`

```shell
cargo install diesel_cli --no-default-features --features "sqlite-bundled" --locked
```

## Run

### Run database migrations

```shell
diesel migration run
```

### Run service

```shell
cargo run
```
