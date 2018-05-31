# Infinity in Rust - A lightweight C++ RDMA library for InfiniBand

[![Crates.io](https://img.shields.io/crates/v/infinity.svg)](https://crates.io/crates/infinity)

Infinity is a simple, powerful, object-oriented abstraction of ibVerbs. The library enables users to build sophisticated applications that use Remote Direct Memory Access (RDMA) without sacrificing performance. It significantly lowers the barrier to get started with RDMA programming. Infinity provides support for two-sided (send/receive) as well as one-sided (read/write/atomic) operations.

This project contains an idiomatic, safe Rust wrapper for the C++ [Infinity](https://github.com/claudebarthels/infinity) library by @claudebarthels.

## Crate

`infinity` is on [crates.io](https://crates.io/crates/infinity). You need ''ibVerbs'' installed for Infinity to build.

## License

Infinity, and the Rust wrapper in this repository, are MIT-licensed.

* [Infinity's license](https://github.com/claudebarthels/infinity/blob/master/LICENSE.txt)
* [This project's license](https://github.com/utaal/infinity-rust/blob/master/LICENSE.txt)
