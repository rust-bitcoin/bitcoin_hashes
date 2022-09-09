[![Status](https://travis-ci.org/Groestlcoin/groestl_hashes.png?branch=master)](https://travis-ci.org/Groestlcoin/groestlcoin_hashes)

# Groestlcoin Hashes Library

This is a simple library which implements the hash functions needed by
Groestlcoin. These are Groestl512, SHA1, SHA256, SHA256d, and RIPEMD160. As an
ancilliary thing, it exposes hexadecimal serialization and deserialization,
since these are needed to display hashes anway.

[Documentation](https://docs.rs/groestlcoin_hashes/)

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.41.1**.

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

