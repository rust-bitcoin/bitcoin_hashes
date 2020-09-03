[![Status](https://travis-ci.org/rust-bitcoin/bitcoin_hashes.png?branch=master)](https://travis-ci.org/rust-bitcoin/bitcoin_hashes)

# Bitcoin Hashes Library

This is a simple, no-dependency library which implements the hash functions
needed by Bitcoin. These are `Sha256`, `Sha256d` (double SHA-256), `Sha256t`
(tagged SHA-256), `Sha512`, `Ripemd160` and `Hash160` (representing 
bitcoin RIPEMD-160 over SHA-256). As an ancillary thing, it exposes hexadecimal 
serialization and deserialization, since these are needed to display hashes 
anyway.

[Documentation](https://docs.rs/bitcoin_hashes/)

## Minimum Supported Rust Version (MSRV)
This library should always compile with any combination of features on **Rust 1.22**.


## Contributions

Contributions are welcome, including additional hash function implementations.
