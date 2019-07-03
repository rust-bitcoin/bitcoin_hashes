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

//! # Error Type
//!

use std::{error, fmt};

/// Hex decoding error
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// non-hexadecimal character
    InvalidChar(u8),
    /// purported hex string had odd length
    OddLengthString(usize),
    /// tried to parse fixed-length hash from a string with the wrong type (expected, got)
    InvalidLength(usize, usize),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidChar(ch) => write!(f, "invalid hex character {}", ch),
            Error::OddLengthString(ell) => write!(f, "odd hex string length {}", ell),
            Error::InvalidLength(ell, ell2) => write!(f, "bad hex string length {} (expected {})", ell2, ell),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        None
    }

    fn description(&self) -> &str {
        match *self {
            Error::InvalidChar(_) => "invalid hex character",
            Error::OddLengthString(_) => "odd hex string length",
            Error::InvalidLength(_, _) => "bad hex string length",
        }
    }
}

