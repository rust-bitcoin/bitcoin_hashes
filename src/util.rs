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

pub use num::endian::*;

/// Circular left-shift a 32-bit word
macro_rules! circular_lshift32 (
    ($shift:expr, $w:expr) => (($w << $shift) | ($w >> (32 - $shift)))
);

/// Circular left-shift a 64-bit word
macro_rules! circular_lshift64 (
    ($shift:expr, $w:expr) => (($w << $shift) | ($w >> (64 - $shift)))
);

#[macro_export]
/// Adds hexadecimal formatting implementation of a trait `$imp` to a given type `$ty`
macro_rules! hex_fmt_impl(
    ($imp:ident, $ty:ident) => (
        hex_fmt_impl!($imp, $ty, );
    );
    ($imp:ident, $ty:ident, $($gen:ident: $gent:ident),*) => (
        impl<$($gen: $gent),*> $crate::core::fmt::$imp for $ty<$($gen),*> {
            fn fmt(&self, f: &mut $crate::core::fmt::Formatter) -> $crate::core::fmt::Result {
                #[allow(unused_imports)]
                use $crate::hex::{format_hex, format_hex_reverse, InnerHex};
                if $ty::<$($gen),*>::DISPLAY_BACKWARD {
                    format_hex_reverse(&self.0, f)
                } else {
                    format_hex(&self.0, f)
                }
            }
        }
    )
);

/// Adds `core::ops::Index` trait implementation to a given type `$ty`
#[macro_export]
macro_rules! index_impl(
    ($ty:ident) => (
        index_impl!($ty, );
    );
    ($ty:ident, $($gen:ident: $gent:ident),*) => (
        impl<$($gen: $gent),*> $crate::core::ops::Index<usize> for $ty<$($gen),*> {
            type Output = u8;
            fn index(&self, index: usize) -> &u8 {
                &self.0[index]
            }
        }

        impl<$($gen: $gent),*> $crate::core::ops::Index<$crate::core::ops::Range<usize>> for $ty<$($gen),*> {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::Range<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl<$($gen: $gent),*> $crate::core::ops::Index<$crate::core::ops::RangeFrom<usize>> for $ty<$($gen),*> {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::RangeFrom<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl<$($gen: $gent),*> $crate::core::ops::Index<$crate::core::ops::RangeTo<usize>> for $ty<$($gen),*> {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::RangeTo<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl<$($gen: $gent),*> $crate::core::ops::Index<$crate::core::ops::RangeFull> for $ty<$($gen),*> {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::RangeFull) -> &[u8] {
                &self.0[index]
            }
        }
    )
);

/// Adds slicing traits implementations to a given type `$ty`
#[macro_export]
macro_rules! borrow_slice_impl(
    ($ty:ident) => (
        borrow_slice_impl!($ty, );
    );
    ($ty:ident, $($gen:ident: $gent:ident),*) => (
        impl<$($gen: $gent),*> $crate::core::borrow::Borrow<[u8]> for $ty<$($gen),*>  {
            fn borrow(&self) -> &[u8] {
                &self[..]
            }
        }

        impl<$($gen: $gent),*> $crate::core::convert::AsRef<[u8]> for $ty<$($gen),*>  {
            fn as_ref(&self) -> &[u8] {
                &self[..]
            }
        }

        impl<$($gen: $gent),*> $crate::core::ops::Deref for $ty<$($gen),*> {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    )
);

macro_rules! engine_input_impl(
    () => (
        #[cfg(not(feature = "fuzztarget"))]
        fn input(&mut self, mut inp: &[u8]) {
            while !inp.is_empty() {
                let buf_idx = self.length % <Self as EngineTrait>::BLOCK_SIZE;
                let rem_len = <Self as EngineTrait>::BLOCK_SIZE - buf_idx;
                let write_len = cmp::min(rem_len, inp.len());

                self.buffer[buf_idx..buf_idx + write_len]
                    .copy_from_slice(&inp[..write_len]);
                self.length += write_len;
                if self.length % <Self as EngineTrait>::BLOCK_SIZE == 0 {
                    self.process_block();
                }
                inp = &inp[write_len..];
            }
        }

        #[cfg(feature = "fuzztarget")]
        fn input(&mut self, inp: &[u8]) {
            for c in inp {
                self.buffer[0] ^= *c;
            }
            self.length += inp.len();
        }
    )
);
