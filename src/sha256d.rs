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

//! # SHA256d

use sha256;
use {Error, Hash};

/// Output of the SHA256d hash function
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Sha256dHash(pub [u8; 32]);

hex_fmt_impl!(Debug, Sha256dHash);
hex_fmt_impl!(Display, Sha256dHash);
hex_fmt_impl!(LowerHex, Sha256dHash);
index_impl!(Sha256dHash);

impl Hash for Sha256dHash {
    type Engine = sha256::Sha256Engine;

    fn engine() -> sha256::Sha256Engine {
        sha256::Sha256Hash::engine()
    }

    fn from_engine(e: sha256::Sha256Engine) -> Sha256dHash {
        let sha2 = sha256::Sha256Hash::from_engine(e);
        let sha2d = sha256::Sha256Hash::hash(&sha2[..]);

        let mut ret = [0; 32];
        ret.copy_from_slice(&sha2d[..]);
        Sha256dHash(ret)
    }

    fn len() -> usize {
        32
    }

    fn block_size() -> usize {
        64
    }

    fn from_slice(sl: &[u8]) -> Result<Sha256dHash, Error> {
        if sl.len() != 32 {
            Err(Error::InvalidLength(Self::len(), sl.len()))
        } else {
            let mut ret = [0; 32];
            ret.copy_from_slice(sl);
            Ok(Sha256dHash(ret))
        }
    }

    fn display_backward() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use sha256d::Sha256dHash;
    use hex::{FromHex, ToHex};
    use Hash;

#[derive(Clone)]
    struct Test {
input: &'static str,
           output: Vec<u8>,
           output_str: &'static str,
    }

#[test]
    fn test() {
        let tests = vec![
            // Test vector copied out of rust-bitcoin
            Test {
                input: "",
                output: vec![
                    0x5d, 0xf6, 0xe0, 0xe2, 0x76, 0x13, 0x59, 0xd3,
                    0x0a, 0x82, 0x75, 0x05, 0x8e, 0x29, 0x9f, 0xcc,
                    0x03, 0x81, 0x53, 0x45, 0x45, 0xf5, 0x5c, 0xf4,
                    0x3e, 0x41, 0x98, 0x3f, 0x5d, 0x4c, 0x94, 0x56, 
                ],
                output_str: "56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d",
            },
        ];

        for test in tests {
            // Hash through high-level API, check hex encoding/decoding
            let hash = Sha256dHash::hash(&test.input.as_bytes());
            assert_eq!(hash, Sha256dHash::from_hex(test.output_str).expect("parse hex"));
            assert_eq!(&hash[..], &test.output[..]);
            assert_eq!(&hash.to_hex(), &test.output_str);

            // Hash through engine, checking that we can input byte by byte
            let mut engine = Sha256dHash::engine();
            for ch in test.input.as_bytes() {
                engine.write(&[*ch]).expect("write to engine");
            }
            let manual_hash = Sha256dHash::from_engine(engine);
            assert_eq!(hash, manual_hash);
        }
    }
}

#[cfg(all(test, feature="unstable"))]
mod benches {
    use std::io::Write;
    use test::Bencher;

    use sha256d::Sha256dHash;
    use Hash;

    #[bench]
    pub fn sha256d_10(bh: & mut Bencher) {
        let mut engine = Sha256dHash::engine();
        let bytes = [1u8; 10];
        bh.iter( || {
            engine.write(&bytes).expect("write");
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    pub fn sha256d_1k(bh: & mut Bencher) {
        let mut engine = Sha256dHash::engine();
        let bytes = [1u8; 1024];
        bh.iter( || {
            engine.write(&bytes).expect("write");
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    pub fn sha256d_64k(bh: & mut Bencher) {
        let mut engine = Sha256dHash::engine();
        let bytes = [1u8; 65536];
        bh.iter( || {
            engine.write(&bytes).expect("write");
        });
        bh.bytes = bytes.len() as u64;
    }

}
