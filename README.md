# Infinity in Rust - A lightweight C++ RDMA library for InfiniBand

[![Docs](https://img.shields.io/badge/docs-master-blue.svg)](https://utaal.github.io/rust-docs/infinity-rust/infinity/) [![Crates.io](https://img.shields.io/crates/v/infinity.svg)](https://crates.io/crates/infinity) [![Build Status](https://travis-ci.org/utaal/infinity-rust.svg?branch=master)](https://travis-ci.org/utaal/infinity-rust)

Infinity is a simple, powerful, object-oriented abstraction of ibVerbs. The library enables users to build sophisticated applications that use Remote Direct Memory Access (RDMA) without sacrificing performance. It significantly lowers the barrier to get started with RDMA programming. Infinity provides support for two-sided (send/receive) as well as one-sided (read/write/atomic) operations.

This project contains an idiomatic, safe Rust wrapper for the C++ [Infinity](https://github.com/claudebarthels/infinity) library by @claudebarthels.

## Crate and documentation

`infinity` is on [crates.io](https://crates.io/crates/infinity). You need ''ibVerbs'' installed for Infinity to build.

The [documentation](https://utaal.github.io/rust-docs/infinity-rust/infinity/) is published by travis for the `master` branch.

## Development

If you're developing on a machine that doesn't have `libibverbs` available, you can use the `utaal/rust-ibverbs` docker image to test your builds as follows:

    docker run --rm -t -v .:/root/infinity-rust utaal/rust-libibverbs bash -c '(cd /root/infinity-rust; cargo build --all)'

The `Dockerfile` for `utaal/rust-libibverbs` is in `docker/`.

## License

Infinity, and the Rust wrapper in this repository, are MIT-licensed.

* [Infinity's license](https://github.com/claudebarthels/infinity/blob/master/LICENSE.txt); Infinity is (C) Claude Barthels, ETH Zurich;
* [This project's license](https://github.com/utaal/infinity-rust/blob/master/LICENSE.txt); this project is (C) Andrea Lattuada, ETH Zurich.
