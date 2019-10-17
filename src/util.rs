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
        impl ::core::fmt::$imp for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                use hex::{format_hex, format_hex_reverse};
                if $ty::DISPLAY_BACKWARD {
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
        impl ::core::ops::Index<usize> for $ty {
            type Output = u8;
            fn index(&self, index: usize) -> &u8 {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::Range<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: ::core::ops::Range<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::RangeFrom<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: ::core::ops::RangeFrom<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::RangeTo<usize>> for $ty {
            type Output = [u8];
            fn index(&self, index: ::core::ops::RangeTo<usize>) -> &[u8] {
                &self.0[index]
            }
        }

        impl ::core::ops::Index<::core::ops::RangeFull> for $ty {
            type Output = [u8];
            fn index(&self, index: ::core::ops::RangeFull) -> &[u8] {
                &self.0[index]
            }
        }
    )
);

macro_rules! borrow_slice_impl(
    ($ty:ty) => (
        impl ::core::borrow::Borrow<[u8]> for $ty {
            fn borrow(&self) -> &[u8] {
                &self[..]
            }
        }

        impl ::core::convert::AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                &self[..]
            }
        }

        impl ::core::ops::Deref for $ty {
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



macro_rules! define_slice_to_be {
    ($name: ident, $type: ty) => {
        #[inline]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::core::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::core::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << (::core::mem::size_of::<$type>() - i - 1)*8;
            }
            res
        }
    }
}
macro_rules! define_slice_to_le {
    ($name: ident, $type: ty) => {
        #[inline]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::core::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::core::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << i*8;
            }
            res
        }
    }
}
macro_rules! define_be_to_array {
    ($name: ident, $type: ty, $byte_len: expr) => {
        #[inline]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            assert_eq!(::core::mem::size_of::<$type>(), $byte_len); // size_of isn't a constfn in 1.22
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> ($byte_len - i - 1)*8) & 0xff) as u8;
            }
            res
        }
    }
}
macro_rules! define_le_to_array {
    ($name: ident, $type: ty, $byte_len: expr) => {
        #[inline]
        pub fn $name(val: $type) -> [u8; $byte_len] {
            assert_eq!(::core::mem::size_of::<$type>(), $byte_len); // size_of isn't a constfn in 1.22
            let mut res = [0; $byte_len];
            for i in 0..$byte_len {
                res[i] = ((val >> i*8) & 0xff) as u8;
            }
            res
        }
    }
}

define_slice_to_be!(slice_to_u32_be, u32);
define_slice_to_be!(slice_to_u64_be, u64);
define_be_to_array!(u32_to_array_be, u32, 4);
define_be_to_array!(u64_to_array_be, u64, 8);

define_slice_to_le!(slice_to_u32_le, u32);
define_slice_to_le!(slice_to_u64_le, u64);
define_le_to_array!(u32_to_array_le, u32, 4);
define_le_to_array!(u64_to_array_le, u64, 8);

#[cfg(test)]
mod test {
    use Hash;
    use sha256;
    use super::*;

    #[test]
    fn borrow_slice_impl_to_vec() {
        // Test that the borrow_slice_impl macro gives to_vec.
        let hash = sha256::Hash::hash(&[3, 50]);
        assert_eq!(hash.to_vec().len(), sha256::Hash::LEN);
    }

    #[test]
    fn endianness_test() {
        assert_eq!(slice_to_u32_be(&[0xde, 0xad, 0xbe, 0xef]), 0xdeadbeef);
        assert_eq!(slice_to_u64_be(&[0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef]), 0x1badcafedeadbeef);
        assert_eq!(u32_to_array_be(0xdeadbeef), [0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(u64_to_array_be(0x1badcafedeadbeef), [0x1b, 0xad, 0xca, 0xfe, 0xde, 0xad, 0xbe, 0xef]);

        assert_eq!(slice_to_u32_le(&[0xef, 0xbe, 0xad, 0xde]), 0xdeadbeef);
        assert_eq!(slice_to_u64_le(&[0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b]), 0x1badcafedeadbeef);
        assert_eq!(u32_to_array_le(0xdeadbeef), [0xef, 0xbe, 0xad, 0xde]);
        assert_eq!(u64_to_array_le(0x1badcafedeadbeef), [0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0xad, 0x1b]);
    }
}
