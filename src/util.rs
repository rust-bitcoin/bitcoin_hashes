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

#[macro_export]
macro_rules! hex_fmt_impl(
    ($imp:ident, $ty:ident) => (
        impl $crate::core::fmt::$imp for $ty {
            fn fmt(&self, f: &mut $crate::core::fmt::Formatter) -> $crate::core::fmt::Result {
                use $crate::hex::{format_hex, format_hex_reverse};
                if $ty::DISPLAY_BACKWARD {
                    format_hex_reverse(&self.0, f)
                } else {
                    format_hex(&self.0, f)
                }
            }
        }
    )
);

#[macro_export]
macro_rules! index_impl(
    ($ty:ty) => (
        impl $crate::core::ops::Index<usize> for $ty {
            type Output = u8;
            fn index(&self, index: usize) -> &u8 {
                &self.0[index]
            }
        }

        impl $crate::core::ops::Index<$crate::core::ops::Range<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::Range<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl $crate::core::ops::Index<$crate::core::ops::RangeFrom<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::RangeFrom<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl $crate::core::ops::Index<$crate::core::ops::RangeTo<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::RangeTo<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl $crate::core::ops::Index<$crate::core::ops::RangeFull> for $ty {
            type Output = [u8];
            fn index(&self, index: $crate::core::ops::RangeFull) -> &[u8] {
                &self.0[index]
            }
        }
    )
);

#[macro_export]
macro_rules! borrow_slice_impl(
    ($ty:ty) => (
        impl $crate::core::borrow::Borrow<[u8]> for $ty {
            fn borrow(&self) -> &[u8] {
                &self[..]
            }
        }

        impl $crate::core::convert::AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                &self[..]
            }
        }

        impl $crate::core::ops::Deref for $ty {
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

#[cfg(test)]
mod test {
    use Hash;
    use sha256;

    #[test]
    fn borrow_slice_impl_to_vec() {
        // Test that the borrow_slice_impl macro gives to_vec.
        let hash = sha256::Hash::hash(&[3, 50]);
        assert_eq!(hash.to_vec().len(), sha256::Hash::LEN);
    }
}
