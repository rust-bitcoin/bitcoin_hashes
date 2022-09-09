// Groestlcoin Hashes Library
// Written in 2020 by
//   Hashengineering <hashengineeringsolutions@gmail.com>
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

//! # Groestl512d implementation (double Groestl512).
//!

use core::str;
use core::ops::Index;
use core::slice::SliceIndex;
use groestl::{Groestl512, Digest};
use crate::{Error, hex};

crate::internal_macros::hash_type! {
    256,
    true,
    "Output of the Groestld hash function.",
    "crate::util::json_hex_string::len_32"
}

/// Engine to compute Groestld hash function.
#[derive(Clone)]
pub struct HashEngine {
    hasher: Groestl512,
    length: usize,
}

impl Default for HashEngine {
    fn default() -> Self {
        HashEngine {
            hasher: Groestl512::new(),
            length: 0,
        }
    }
}

// This will handle a single Groestl512 hash
impl crate::HashEngine for HashEngine {
    type MidState = [u8; 64];

    // this is not supported by Groestl512
    #[cfg(not(fuzzing))]
    fn midstate(&self) -> [u8; 64] {
        static RET: [u8; 64] = [0; 64];
        RET
    }

    #[cfg(fuzzing)]
    fn midstate(&self) -> [u8; 64] {
        let ret = [0; 64];
        // this is not supported by Groestl512
        ret
    }

    const BLOCK_SIZE: usize = 64;

    fn input(&mut self, inp: &[u8]) {
        self.hasher.update(inp);
        self.length += inp.len()
    }

    fn n_bytes_hashed(&self) -> usize {
        self.length
    }
}




fn from_engine(e: HashEngine) -> Hash {
    let first = e.hasher.finalize();

    let mut groestl_engine2 = Groestl512::new();
    groestl_engine2.update(first);
    let result = groestl_engine2.finalize();

    // use the first 32 bytes
    let mut ret = [0; 32];
    //for x in 0..32 {
    //    ret[x] = result.as_slice()[x]; // x: i32
    //}
    ret[..32].clone_from_slice(&result.as_slice()[..32]);

    Hash(ret)
}


#[cfg(test)]
mod tests {
    use crate::{groestld, Hash, HashEngine};

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn test() {
        use crate::hex::{FromHex, ToHex};

        #[derive(Clone)]
        struct Test {
            input: &'static str,
            output: Vec<u8>,
            output_str: &'static str,
        }

        let tests = vec![
            // Examples from wikipedia
            Test {
                input: "",
                output: vec![
                    0xfd, 0xfb, 0x14, 0xd3, 0x86, 0xc6, 0xdf, 0xf8,
                    0x57, 0x15, 0xc5, 0x0e, 0xfb, 0x82, 0x6c, 0x43,
                    0xe0, 0x42, 0x05, 0xb1, 0x84, 0x10, 0x49, 0x7a,
                    0xa4, 0x7f, 0x12, 0x1e, 0xce, 0xb3, 0xa6, 0x5e,

                ],
                output_str: "5ea6b3ce1e127fa47a491084b10542e0436c82fb0ec51557f8dfc686d314fbfd"
            },
            Test {
                input: "The quick brown fox jumps over the lazy dog",
                output: vec![
                    0x12, 0x09, 0xd2, 0x29, 0xcf, 0xc9, 0xd7, 0xd6,
                    0x71, 0x13, 0x69, 0xe2, 0xd7, 0xf3, 0x69, 0xb0,
                    0xef, 0xc1, 0x45, 0x9a, 0x9d, 0x40, 0x7c, 0xbf,
                    0xc7, 0xda, 0xf4, 0xf5, 0x42, 0x09, 0x34, 0x7f,
                ],
                output_str: "7f340942f5f4dac7bf7c409d9a45c1efb069f3d7e2691371d6d7c9cf29d20912",
            },
            Test {
                input: "The quick brown fox jumps over the lazy dog.",
                output: vec![
                    0xf3, 0x32, 0x2d, 0xae, 0x35, 0x14, 0x73, 0xff,
                    0xf3, 0x42, 0x27, 0x8c, 0x15, 0x20, 0x2b, 0x0f,
                    0x71, 0x3c, 0x4c, 0x24, 0xde, 0x61, 0xa3, 0x52,
                    0x57, 0x00, 0xc1, 0x45, 0xc3, 0x45, 0x32, 0x77,
                ],
                output_str: "773245c345c1005752a361de244c3c710f2b20158c2742f3ff731435ae2d32f3",
            },
        ];

        for test in tests {
            // Hash through high-level API, check hex encoding/decoding
            let hash = groestld::Hash::hash(&test.input.as_bytes());
            assert_eq!(hash, groestld::Hash::from_hex(test.output_str).expect("parse hex"));
            assert_eq!(&hash[..], &test.output[..]);
            assert_eq!(&hash.to_hex(), &test.output_str);

            // Hash through engine, checking that we can input byte by byte
            let mut engine = groestld::Hash::engine();
            for ch in test.input.as_bytes() {
                engine.input(&[*ch]);
            }
            let manual_hash = groestld::Hash::from_engine(engine);
            assert_eq!(hash, manual_hash);
            assert_eq!(hash.into_inner()[..].as_ref(), test.output.as_slice());
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn sha256_serde() {
        use serde_test::{Configure, Token, assert_tokens};
        //use {groestld, Hash};

        static HASH_BYTES: [u8; 32] = [
            0xef, 0x53, 0x7f, 0x25, 0xc8, 0x95, 0xbf, 0xa7,
            0x82, 0x52, 0x65, 0x29, 0xa9, 0xb6, 0x3d, 0x97,
            0xaa, 0x63, 0x15, 0x64, 0xd5, 0xd7, 0x89, 0xc2,
            0xb7, 0x65, 0x44, 0x8c, 0x86, 0x35, 0xfb, 0x6c,
        ];

        let hash = groestld::Hash::from_slice(&HASH_BYTES).expect("right number of bytes");
        assert_tokens(&hash.compact(), &[Token::BorrowedBytes(&HASH_BYTES[..])]);
        assert_tokens(&hash.readable(), &[Token::Str("6cfb35868c4465b7c289d7d5641563aa973db6a929655282a7bf95c8257f53ef")]);
    }

    #[cfg(target_arch = "wasm32")]
    mod wasm_tests {
        extern crate wasm_bindgen_test;
        use super::*;
        use self::wasm_bindgen_test::*;
        #[wasm_bindgen_test]
        fn groestld_tests() {
            test();
        }
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    use test::Bencher;

    use crate::{Hash, HashEngine, groestld};

    #[bench]
    pub fn groestl512_10(bh: &mut Bencher) {
        let mut engine = groestld::Hash::engine();
        let bytes = [1u8; 10];
        bh.iter( || {
            engine.input(&bytes);
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    pub fn groestl512_1k(bh: &mut Bencher) {
        let mut engine = groestld::Hash::engine();
        let bytes = [1u8; 1024];
        bh.iter( || {
            engine.input(&bytes);
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    pub fn groestl512_64k(bh: &mut Bencher) {
        let mut engine = groestld::Hash::engine();
        let bytes = [1u8; 65536];
        bh.iter( || {
            engine.input(&bytes);
        });
        bh.bytes = bytes.len() as u64;
    }

}
