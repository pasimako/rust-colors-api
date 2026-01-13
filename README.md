# Rust Colors API

A minimal Rust API server built with Axum.

This repository contains the exact code built step-by-step in my tutorial at [ubuverse.com/introduction-to-restful-api-development-with-rust-for-nodejs-developers/](https://ubuverse.com/introduction-to-restful-api-development-with-rust-for-nodejs-developers/).

## Running

```shell
cargo run
```

## Testing

```shell
cargo test
```

## Endpoints

### `GET /colors`

Returns all colors.

#### Response

```json
{
  "colors": ["RED", "GREEN", "BLUE"]
}
```

### `POST /colors`

Adds a new color.

#### Request

```json
{
  "color": "BLUE"
}
```

#### Response

```json
{
  "colors": ["BLUE"]
}
```
