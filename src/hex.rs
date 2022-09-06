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

//! Hex encoding and decoding.
//!

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::alloc::string::String;

#[cfg(any(test, feature = "std"))]
use std::io;
#[cfg(all(not(test), not(feature = "std"), feature = "core2"))]
use core2::io;

use core::{fmt, str};

/// Hex decoding error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// Non-hexadecimal character.
    InvalidChar(u8),
    /// Purported hex string had odd length.
    OddLengthString(usize),
    /// Tried to parse fixed-length hash from a string with the wrong type (expected, got).
    InvalidLength(usize, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidChar(ch) => write!(f, "invalid hex character {}", ch),
            Error::OddLengthString(ell) => write!(f, "odd hex string length {}", ell),
            Error::InvalidLength(ell, ell2) => write!(f, "bad hex string length {} (expected {})", ell2, ell),
        }
    }
}

/// Iterator over a hex-encoded string slice which decodes hex and yields bytes.
pub struct HexIterator<'a> {
    /// The `Bytes` iterator whose next two bytes will be decoded to yield
    /// the next byte.
    iter: str::Bytes<'a>,
}

impl<'a> HexIterator<'a> {
    /// Constructs a new `HexIterator` from a string slice.
    ///
    /// # Errors
    ///
    /// If the input string is of odd length.
    pub fn new(s: &'a str) -> Result<HexIterator<'a>, Error> {
        if s.len() % 2 != 0 {
            Err(Error::OddLengthString(s.len()))
        } else {
            Ok(HexIterator { iter: s.bytes() })
        }
    }
}

fn chars_to_hex(hi: u8, lo: u8) -> Result<u8, Error> {
    let hih = (hi as char)
        .to_digit(16)
        .ok_or(Error::InvalidChar(hi))?;
    let loh = (lo as char)
        .to_digit(16)
        .ok_or(Error::InvalidChar(lo))?;

    let ret = (hih << 4) + loh;
    Ok(ret as u8)
}

impl<'a> Iterator for HexIterator<'a> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Result<u8, Error>> {
        let hi = self.iter.next()?;
        let lo = self.iter.next().unwrap();
        Some(chars_to_hex(hi, lo))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.iter.size_hint();
        (min / 2, max.map(|x| x / 2))
    }
}

#[cfg(any(feature = "std", feature = "core2"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "core2"))))]
impl<'a> io::Read for HexIterator<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0usize;
        for dst in buf {
            match self.next() {
                Some(Ok(src)) => {
                    *dst = src;
                    bytes_read += 1;
                },
                _ => break,
            }
        }
        Ok(bytes_read)
    }
}

impl<'a> DoubleEndedIterator for HexIterator<'a> {
    fn next_back(&mut self) -> Option<Result<u8, Error>> {
        let lo = self.iter.next_back()?;
        let hi = self.iter.next_back().unwrap();
        Some(chars_to_hex(hi, lo))
    }
}

impl<'a> ExactSizeIterator for HexIterator<'a> {}

/// Outputs hex into an object implementing `fmt::Write`.
///
/// This is usually more efficient than going through a `String` using [`std::string::ToString`].
pub fn format_hex(data: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
    let prec = f.precision().unwrap_or(2 * data.len());
    let width = f.width().unwrap_or(2 * data.len());
    for _ in (2 * data.len())..width {
        f.write_str("0")?;
    }
    for ch in data.iter().take(prec / 2) {
        write!(f, "{:02x}", *ch)?;
    }
    if prec < 2 * data.len() && prec % 2 == 1 {
        write!(f, "{:x}", data[prec / 2] / 16)?;
    }
    Ok(())
}

/// Outputs hex in reverse order.
///
/// Used for `sha256d::Hash` whose standard hex encoding has the bytes reversed.
pub fn format_hex_reverse(data: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
    let prec = f.precision().unwrap_or(2 * data.len());
    let width = f.width().unwrap_or(2 * data.len());
    for _ in (2 * data.len())..width {
        f.write_str("0")?;
    }
    for ch in data.iter().rev().take(prec / 2) {
        write!(f, "{:02x}", *ch)?;
    }
    if prec < 2 * data.len() && prec % 2 == 1 {
        write!(f, "{:x}", data[data.len() - 1 - prec / 2] / 16)?;
    }
    Ok(())
}

/// A struct implementing [`io::Write`] that converts what's written to it into
/// a hex String.
///
/// If you already have the data to be converted in a `Vec<u8>` use [`ToString`]
/// but if you have an encodable object, by using this you avoid the
/// serialization to `Vec<u8>` by going directly to `String`.
///
/// Note that to achieve better perfomance than [`ToString`] the struct must be
/// created with the right `capacity` of the final hex string so that the inner
/// `String` doesn't re-allocate.
#[cfg(any(test, feature = "std", feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(any(test, feature = "std", feature = "alloc"))))]
pub struct HexWriter(String);

#[cfg(any(test, feature = "std", feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(any(test, feature = "std", feature = "alloc"))))]
impl HexWriter {
    /// Creates a new [`HexWriter`] with the `capacity` of the inner `String`
    /// that will contain final hex value.
    pub fn new(capacity: usize) -> Self {
        HexWriter(String::with_capacity(capacity))
    }

    /// Returns the resulting hex string.
    pub fn result(self) -> String {
        self.0
    }
}

#[cfg(any(test, feature = "std", feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(any(test, feature = "std", feature = "alloc"))))]
impl io::Write for HexWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use core::fmt::Write;
        for ch in buf {
            write!(self.0, "{:02x}", ch).expect("writing to string");
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::fmt;
    use std::io::Write;

    fn to_hex(bytes: &[u8]) -> String {
        use core::fmt::Write;
        let mut ret = String::with_capacity(2 * bytes.len());
        for ch in bytes {
            write!(ret, "{:02x}", ch).expect("writing to string");
        }
        ret
    }

    fn from_hex(hex: &str) -> Vec<u8> {
        assert!(hex.len() % 2 == 0, "uneven length hex string");

        let mut v = vec![];

        let mut b = 0;
        let mut idx = 0;
        for c in hex.bytes() {
            b <<= 4;
            match c {
                b'A'..=b'F' => b |= c - b'A' + 10,
                b'a'..=b'f' => b |= c - b'a' + 10,
                b'0'..=b'9' => b |= c - b'0',
                _ => panic!("invalid hex character"),
            }
            if (idx & 1) == 1 {
                v.push(b);
                b = 0;
            }
            idx += 1;
        }
        v
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn hex_roundtrip() {
        let expected = "0123456789abcdef";
        let expected_up = "0123456789ABCDEF";

        let parse = from_hex(expected);
        assert_eq!(parse, vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        let ser = to_hex(&parse);
        assert_eq!(ser, expected);

        let parse = from_hex(expected_up);
        assert_eq!(parse, vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        let ser = to_hex(&parse);
        assert_eq!(ser, expected);

        let parse = from_hex(expected_up);
        assert_eq!(parse, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        let ser = to_hex(&parse);
        assert_eq!(ser, expected);
    }

    #[test]
    fn hex_truncate() {
        struct HexBytes(Vec<u8>);
        impl fmt::LowerHex for HexBytes {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                format_hex(&self.0, f)
            }
        }

        let bytes = HexBytes(vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        assert_eq!(
            format!("{:x}", bytes),
            "0102030405060708090a"
        );

        for i in 0..20 {
            assert_eq!(
                format!("{:.prec$x}", bytes, prec = i),
                &"0102030405060708090a"[0..i]
            );
        }

        assert_eq!(
            format!("{:25x}", bytes),
            "000000102030405060708090a"
        );
        assert_eq!(
            format!("{:26x}", bytes),
            "0000000102030405060708090a"
        );
    }

    #[test]
    fn hex_truncate_rev() {
        struct HexBytes(Vec<u8>);
        impl fmt::LowerHex for HexBytes {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                format_hex_reverse(&self.0, f)
            }
        }

        let bytes = HexBytes(vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        assert_eq!(
            format!("{:x}", bytes),
            "0a090807060504030201"
        );

        for i in 0..20 {
            assert_eq!(
                format!("{:.prec$x}", bytes, prec = i),
                &"0a090807060504030201"[0..i]
            );
        }

        assert_eq!(
            format!("{:25x}", bytes),
            "000000a090807060504030201"
        );
        assert_eq!(
            format!("{:26x}", bytes),
            "0000000a090807060504030201"
        );
    }

    #[test]
    fn hex_writer() {
        let vec: Vec<_>  = (0u8..32).collect();
        let mut writer = HexWriter::new(64);
        writer.write_all(&vec[..]).unwrap();
        let want = to_hex(&vec);
        assert_eq!(writer.result(), want);
    }
}


#[cfg(all(test, feature="unstable"))]
mod benches {
    use test::{Bencher, black_box};
    use super::HexWriter;
    use std::io::Write;
    use crate::{sha256, Hash};

    #[bench]
    fn bench_to_string(bh: &mut Bencher) {
        let hash = sha256::Hash::hash(&[0; 1]);
        bh.iter(|| {
            black_box(hash.to_string());
        })
    }


    #[bench]
    fn bench_to_string_writer(bh: &mut Bencher) {
        let hash = sha256::Hash::hash(&[0; 1]);
        bh.iter(|| {
            let mut writer = HexWriter::new(64);
            writer.write_all(hash.as_inner()).unwrap();
            black_box(writer.result());
        })
    }
}
