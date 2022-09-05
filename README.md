[![Status](https://travis-ci.org/rust-bitcoin/bitcoin_hashes.png?branch=master)](https://travis-ci.org/rust-bitcoin/bitcoin_hashes)

# Bitcoin Hashes Library

This is a simple, no-dependency library which implements the hash functions
needed by Bitcoin. These are SHA1, SHA256, SHA256d, SHA512, and RIPEMD160. As an
ancilliary thing, it exposes hexadecimal serialization and deserialization,
since these are needed to display hashes anway.

[Documentation](https://docs.rs/bitcoin_hashes/)

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.41.1**.
The one exception is the `schemars` feature which has no MSRV and should not be used
by users who expect stability from their libraries.

### Dependency pinning

Some dependencies break the MSRV build (only when building with `schemars` feature), to build with
Rust 1.41.1 you may need to pin as follows:
```bash

    cargo update --package schemars --precise 0.8.3
    cargo update --package dyn-clone --precise 1.0.7
```

## Contributions

Contributions are welcome, including additional hash function implementations.

### Githooks

To assist devs in catching errors _before_ running CI we provide some githooks. If you do not
already have locally configured githooks you can use the ones in this repository by running, in the
root directory of the repository:
```
git config --local core.hooksPath githooks/
```

Alternatively add symlinks in your `.git/hooks` directory to any of the githooks we provide.

### Running Benchmarks

We use a custom Rust compiler configuration conditional to guard the bench mark code. To run the
bench marks use: `RUSTFLAGS='--cfg=bench' cargo +nightly bench`.

