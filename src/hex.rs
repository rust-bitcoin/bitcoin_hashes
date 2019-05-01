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

//! # Hex encoding and decoding
//!

use std::fmt;
use {Error, Hash};

/// Trait for objects that can be serialized as hex strings
pub trait ToHex {
    /// Hex representation of the object
    fn to_hex(&self) -> String;
}

/// Trait for objects that can be deserialized from hex strings
pub trait FromHex: Sized {
    /// Produce an object from a byte iterator
    fn from_byte_iter<I>(iter: I) -> Result<Self, Error>
        where I: Iterator<Item=Result<u8, Error>> +
            ExactSizeIterator +
            DoubleEndedIterator;

    /// Produce an object from a hex string
    fn from_hex(s: &str) -> Result<Self, Error> {
        if s.len() % 2 == 1 {
            Err(Error::OddLengthString(s.len()))
        } else {
            Self::from_byte_iter(HexIterator {
                sl: s,
            })
        }
    }
}

impl<T: fmt::LowerHex> ToHex for T {
    /// Outputs the hash in hexadecimal form
    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }
}

impl<T: Hash> FromHex for T {
    fn from_byte_iter<I>(iter: I) -> Result<Self, Error>
        where I: Iterator<Item=Result<u8, Error>> +
            ExactSizeIterator +
            DoubleEndedIterator,
    {
        let inner;
        if Self::DISPLAY_BACKWARD {
            inner = T::Inner::from_byte_iter(iter.rev())?;
        } else {
            inner = T::Inner::from_byte_iter(iter)?;
        }
        Ok(Hash::from_inner(inner))
    }
}

/// Iterator over a hex-encoded string slice which decodes hex and yields bytes.
pub struct HexIterator<'a> {
    /// The slice whose first two characters will be decoded to yield the next byte
    pub sl: &'a str
}

impl<'a> Iterator for HexIterator<'a> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Result<u8, Error>> {
        if self.sl.len() % 2 == 1 {
            Some(Err(Error::OddLengthString(self.sl.len())))
        } else if self.sl.is_empty() {
            None
        } else {
            let (hi, lo) = {
                let mut iter = self.sl.chars();
                let hi = iter.next().unwrap();
                let lo = iter.next().unwrap();
                match (hi.to_digit(16), lo.to_digit(16)) {
                    (Some(hi), Some(lo)) => (hi, lo),
                    (None, _) => return Some(Err(Error::InvalidChar(hi))),
                    (_, None) => return Some(Err(Error::InvalidChar(lo))),
                }
            };
            let ret = (hi << 4) + lo;
            self.sl = &self.sl[2..];
            Some(Ok(ret as u8))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.sl.len() / 2;
        (len, Some(len))
    }
}

impl<'a> DoubleEndedIterator for HexIterator<'a> {
    fn next_back(&mut self) -> Option<Result<u8, Error>> {
        let current_len = self.sl.len();
        if current_len % 2 == 1 {
            Some(Err(Error::OddLengthString(current_len)))
        } else if self.sl.is_empty() {
            None
        } else {
            let (hi, lo) = {
                let mut iter = self.sl.chars().rev();
                let lo = iter.next().unwrap();
                let hi = iter.next().unwrap();
                match (hi.to_digit(16), lo.to_digit(16)) {
                    (Some(hi), Some(lo)) => (hi, lo),
                    (None, _) => return Some(Err(Error::InvalidChar(hi))),
                    (_, None) => return Some(Err(Error::InvalidChar(lo))),
                }
            };
            let ret = (hi << 4) + lo;
            self.sl = &self.sl[..current_len - 2];
            Some(Ok(ret as u8))
        }
    }

}

impl<'a> ExactSizeIterator for HexIterator<'a> {
}

/// Output hex into an object implementing `fmt::Write`, which is usually more
/// efficient than going through a `String` using `ToHex`.
pub fn format_hex(data: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
    let prec = f.precision().unwrap_or(2 * data.len());
    let width = f.width().unwrap_or(2 * data.len());
    for _ in (2 * data.len())..width {
        f.write_str("0")?;
    }
    for ch in data.into_iter().take(prec / 2) {
        write!(f, "{:02x}", *ch)?;
    }
    if prec < 2 * data.len() && prec % 2 == 1 {
        write!(f, "{:x}", data[prec / 2] / 16)?;
    }
    Ok(())
}

/// Output hex in reverse order; used for Sha256dHash whose standard hex encoding
/// has the bytes reversed.
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

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        use std::fmt::Write;
        let mut ret = String::with_capacity(2 * self.len());
        for ch in self {
            write!(ret, "{:02x}", ch).expect("writing to string");
        }
        ret
    }
}

impl FromHex for Vec<u8> {
    fn from_byte_iter<I>(iter: I) -> Result<Self, Error>
        where I: Iterator<Item=Result<u8, Error>> +
            ExactSizeIterator +
            DoubleEndedIterator,
    {
        iter.collect()
    }
}

macro_rules! impl_fromhex_array {
    ($len:expr) => {
        impl FromHex for [u8; $len] {
            fn from_byte_iter<I>(iter: I) -> Result<Self, Error>
                where I: Iterator<Item=Result<u8, Error>> +
                    ExactSizeIterator +
                    DoubleEndedIterator,
            {
                if iter.len() == $len {
                    let mut ret = [0; $len];
                    for (n, byte) in iter.enumerate() {
                        ret[n] = byte?;
                    }
                    Ok(ret)
                } else {
                    Err(Error::InvalidLength(2 * $len, 2 * iter.len()))
                }
            }
        }
    }
}

impl_fromhex_array!(2);
impl_fromhex_array!(4);
impl_fromhex_array!(6);
impl_fromhex_array!(8);
impl_fromhex_array!(10);
impl_fromhex_array!(12);
impl_fromhex_array!(14);
impl_fromhex_array!(16);
impl_fromhex_array!(20);
impl_fromhex_array!(24);
impl_fromhex_array!(28);
impl_fromhex_array!(32);
impl_fromhex_array!(33);
impl_fromhex_array!(64);
impl_fromhex_array!(65);
impl_fromhex_array!(128);
impl_fromhex_array!(256);
impl_fromhex_array!(384);
impl_fromhex_array!(512);

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::{format_hex, format_hex_reverse, ToHex, FromHex};
    use Error;

    #[test]
    fn hex_roundtrip() {
        let expected = "0123456789abcdef";
        let expected_up = "0123456789ABCDEF";

        let parse: Vec<u8> = FromHex::from_hex(expected).expect("parse lowercase string");
        assert_eq!(parse, vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        let ser = parse.to_hex();
        assert_eq!(ser, expected);

        let parse: Vec<u8> = FromHex::from_hex(expected_up).expect("parse uppercase string");
        assert_eq!(parse, vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        let ser = parse.to_hex();
        assert_eq!(ser, expected);

        let parse: [u8; 8] = FromHex::from_hex(expected_up).expect("parse uppercase string");
        assert_eq!(parse, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        let ser = parse.to_hex();
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
    fn hex_error() {
        let oddlen = "0123456789abcdef0";
        let badchar1 = "Z123456789abcdef";
        let badchar2 = "012Y456789abcdeb";
        let badchar3 = "«23456789abcdef";

        assert_eq!(
            Vec::<u8>::from_hex(oddlen),
            Err(Error::OddLengthString(17))
        );
        assert_eq!(
            <[u8; 4]>::from_hex(oddlen),
            Err(Error::OddLengthString(17))
        );
        assert_eq!(
            <[u8; 8]>::from_hex(oddlen),
            Err(Error::OddLengthString(17))
        );
        assert_eq!(
            Vec::<u8>::from_hex(badchar1),
            Err(Error::InvalidChar('Z'))
        );
        assert_eq!(
            Vec::<u8>::from_hex(badchar2),
            Err(Error::InvalidChar('Y'))
        );
        assert_eq!(
            Vec::<u8>::from_hex(badchar3),
            Err(Error::InvalidChar('«'))
        );
    }
}

