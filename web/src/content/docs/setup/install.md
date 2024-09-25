---
title: Installation
description: Guide how to install commitlint to your project
---

## Using `cargo` CLI

Commitlint is written in Rust so you can install it using `cargo` CLI:

```console
cargo install commitlint-rs
```

After that, you will be able to run the `commitlint` command.

```console
commitlint --help
```

## Using Cargo Binary Install

You can also use Binstall ([cargo-bins/cargo-binstall](https://github.com/cargo-bins/cargo-binstall)) to install the CLI

```console
cargo binstall commitlint-rs
```

## Using Docker

Commitlint is also available as a Docker image.
You can pull it from [Docker Hub](https://hub.docker.com/repository/docker/1915keke/commitlint).

```console
docker run 1915keke/commitlint
```

See all available tags [here](https://hub.docker.com/repository/docker/1915keke/commitlint/tags?page=1&ordering=last_updated).
