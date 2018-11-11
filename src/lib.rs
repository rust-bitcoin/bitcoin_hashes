// Bitcoin Hashes Library
// Written in 2018 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Hashes Library
//!
//! This is a simple, no-dependency library which implements the hash functions
//! needed by Bitcoin. These are SHA256, SHA256d, and RIPEMD160. As an ancillary
//! thing, it exposes hexadecimal serialization and deserialization, since these
//! are needed to display hashes anway.
//!

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#[cfg(all(test, feature = "unstable"))] extern crate test;

extern crate byteorder;

#[macro_use] mod util;
pub mod error;
pub mod hex;
pub mod hash160;
pub mod hmac;
pub mod ripemd160;
pub mod sha1;
pub mod sha256;
pub mod sha512;
pub mod sha256d;

use std::{fmt, io, ops};

pub use hmac::{Hmac, HmacEngine};
pub use error::Error;

/// A hashing engine which bytes can be serialized into. It is expected
/// to implement the `io::Write` trait, but to never return errors under
/// any conditions.
pub trait HashEngine: Clone + io::Write {
    /// Byte array representing the internal state of the hash engine
    type MidState;

    /// Outputs the midstate of the hash engine. This function should not be
    /// used directly unless you really know what you're doing.
    fn midstate(&self) -> Self::MidState;

    /// Add data to the hash engine without any error return type to deal with
    #[inline(always)]
    fn input(&mut self, data: &[u8]) {
        self.write_all(data).expect("hash returned error");
    }
}

/// Trait which applies to hashes of all types
pub trait Hash: Copy + Clone + PartialEq + Eq +
    fmt::Debug + fmt::Display + fmt::LowerHex +
    ops::Index<ops::RangeFull, Output = [u8]> +
    ops::Index<ops::RangeFrom<usize>, Output = [u8]> +
    ops::Index<ops::RangeTo<usize>, Output = [u8]> +
    ops::Index<ops::Range<usize>, Output = [u8]> +
    ops::Index<usize, Output = u8> +
    hex::ToHex
{
    /// A hashing engine which bytes can be serialized into. It is expected
    /// to implement the `io::Write` trait, and to never return errors under
    /// any conditions.
    type Engine: HashEngine;

    /// Construct a new engine
    fn engine() -> Self::Engine;

    /// Produce a hash from the current state of a given engine
    fn from_engine(e: Self::Engine) -> Self;

    /// Length of the hash, in bytes
    fn len() -> usize;

    /// Length of the hash's internal block size, in bytes
    fn block_size() -> usize;

    /// Copies a byte slice into a hash object
    fn from_slice(sl: &[u8]) -> Result<Self, Error>;

    /// Hashes some bytes
    fn hash(data: &[u8]) -> Self {
        use std::io::Write;

        let mut engine = Self::engine();
        engine.write_all(data).unwrap();
        Self::from_engine(engine)
    }

    /// Flag indicating whether user-visible serializations of this hash
    /// should be backward. For some reason Satoshi decided this should be
    /// true for `Sha256dHash`, so here we are.
    fn display_backward() -> bool { false }
}

