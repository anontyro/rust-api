# API Documentation

## Table of Contents

1. [Introduction](#introduction)

## Introduction

This is the api for the project. It is a REST api that is used to interact with the database. It aims to provide verious API interfaces used mostly for smart home interactions.

The api is written in Rust and uses the [axum](https://docs.rs/axum/latest/axum) framework. The api is fully containerised and can be run using docker-compose.

### Setup

The api is written in Rust and uses the [axum](https://docs.rs/axum/latest/axum). The api can be run locally using cargo or with docker. inital setup:

```bash
cargo build
```

### Running on development

Locally the api can be run using cargo, this will require Rust to be installed locally with cargo using:

```bash
cargo run
```

then the api can be accessed at `http://localhost:3000` using the standard routes given in the `main.rs` file.

### Running with docker

The project is fully containerised and can be run using docker-compose. This will require docker and docker-compose to be installed locally. To run the api using docker-compose use:

```bash
docker-compose build
docker-compose up
```

then the api can be accessed at `http://localhost:3000` using the standard routes given in the `main.rs` file.
