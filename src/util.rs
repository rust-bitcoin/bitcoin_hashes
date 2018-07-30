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

/// Circular left-shift a 32-bit word
macro_rules! circular_lshift32 (
    ($shift:expr, $w:expr) => (($w << $shift) | ($w >> (32 - $shift)))
);

macro_rules! circular_lshift64 (
    ($shift:expr, $w:expr) => (($w << $shift) | ($w >> (64 - $shift)))
);

macro_rules! hex_fmt_impl(
    ($imp:ident, $ty:ident) => (
        impl ::std::fmt::$imp for $ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use hex::{format_hex, format_hex_reverse};
                if $ty::display_backward() {
                    format_hex_reverse(&self.0, f)
                } else {
                    format_hex(&self.0, f)
                }
            }
        }
    )
);

macro_rules! index_impl(
    ($ty:ty) => (
        impl ::std::ops::Index<usize> for $ty {
            type Output = u8;
            fn index(&self, index: usize) -> &u8 {
                &self.0[index]
            }
        }

        impl ::std::ops::Index<::std::ops::Range<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: ::std::ops::Range<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl ::std::ops::Index<::std::ops::RangeFrom<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: ::std::ops::RangeFrom<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl ::std::ops::Index<::std::ops::RangeTo<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: ::std::ops::RangeTo<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl ::std::ops::Index<::std::ops::RangeFull> for $ty {
            type Output = [u8];
            fn index(&self, index: ::std::ops::RangeFull) -> &[u8] {
                &self.0[index]
            }
        }
    )
);

