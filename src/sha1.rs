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

//! # SHA1

use std::io;

use byteorder::{ByteOrder, BigEndian};

use {Error, Hash, HashEngine};

const BLOCK_SIZE: usize = 64;

/// Engine to compute SHA1 hash function
pub struct Sha1Engine {
    buffer: [u8; BLOCK_SIZE],
    h: [u32; 5],
    length: usize,
}

impl Clone for Sha1Engine {
    fn clone(&self) -> Sha1Engine {
        Sha1Engine {
            h: self.h,
            length: self.length,
            buffer: self.buffer,
        }
    }
}

impl HashEngine for Sha1Engine {
    type MidState = [u8; 20];

    fn midstate(&self) -> [u8; 20] {
        let mut ret = [0; 20];
        BigEndian::write_u32_into(&self.h, &mut ret);
        ret
    }
}

/// Output of the SHA1 hash function
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Sha1Hash(pub [u8; 20]);

hex_fmt_impl!(Debug, Sha1Hash);
hex_fmt_impl!(Display, Sha1Hash);
hex_fmt_impl!(LowerHex, Sha1Hash);
index_impl!(Sha1Hash);

impl Hash for Sha1Hash {
    type Engine = Sha1Engine;

    fn engine() -> Sha1Engine {
        Sha1Engine {
            h: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0],
            length: 0,
            buffer: [0; BLOCK_SIZE],
        }
    }

    fn from_engine(mut e: Sha1Engine) -> Sha1Hash {
        use std::io::Write;
        use byteorder::WriteBytesExt;

        // pad buffer with a single 1-bit then all 0s, until there are exactly 8 bytes remaining
        let data_len = e.length as u64;

        let zeroes = [0; BLOCK_SIZE - 8];
        e.write(&[0x80]).unwrap();
        if e.length % BLOCK_SIZE > zeroes.len() {
            e.write(&zeroes).unwrap();
        }
        let pad_length = zeroes.len() - (e.length % BLOCK_SIZE);
        e.write(&zeroes[..pad_length]).unwrap();
        debug_assert_eq!(e.length % BLOCK_SIZE, zeroes.len());

        e.write_u64::<BigEndian>(8 * data_len).unwrap();
        debug_assert_eq!(e.length % BLOCK_SIZE, 0);

        Sha1Hash(e.midstate())
    }

    fn len() -> usize {
        20
    }

    fn block_size() -> usize {
        64
    }

    fn from_slice(sl: &[u8]) -> Result<Sha1Hash, Error> {
        if sl.len() != 20 {
            Err(Error::InvalidLength(Self::len(), sl.len()))
        } else {
            let mut ret = [0; 20];
            ret.copy_from_slice(sl);
            Ok(Sha1Hash(ret))
        }
    }
}

impl io::Write for Sha1Engine {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, mut inp: &[u8]) -> io::Result<usize> {
        let ret = Ok(inp.len());

        while !inp.is_empty() {
            let buf_idx = self.length % BLOCK_SIZE;
            let rem_len = BLOCK_SIZE - buf_idx;
            let write_len;

            if inp.len() >= rem_len {
                write_len = rem_len;
            } else {
                write_len = inp.len();
            }

            self.buffer[buf_idx..buf_idx + write_len].copy_from_slice(&inp[..write_len]);
            inp = &inp[write_len..];
            self.length += write_len;
            if self.length % BLOCK_SIZE == 0 {
                self.process_block();
            }
        }
        ret
    }
}

impl Sha1Engine {
    // Basic unoptimized algorithm from Wikipedia
    fn process_block(&mut self) {
        debug_assert_eq!(self.buffer.len(), BLOCK_SIZE);

        let mut w = [0u32; 80];
        BigEndian::read_u32_into(&self.buffer, &mut w[0..16]);
        for i in 16..80 {
            w[i] = circular_lshift32!(1, w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]);
        }

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];

        for i in 0..80 {
            let (f, k) = match i {
                 0...19 => ((b & c) | (!b & d), 0x5a827999),
                20...39 => (b ^ c ^ d, 0x6ed9eba1),
                40...59 => ((b & c) | (b & d) | (c & d), 0x8f1bbcdc),
                60...79 => (b ^ c ^ d, 0xca62c1d6),
                _ => unreachable!()
            };

            let new_a = circular_lshift32!(5, a).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d;
            d = c;
            c = circular_lshift32!(30, b);
            b = a;
            a = new_a;
        }

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use sha1::Sha1Hash;
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
            // Examples from wikipedia
            Test {
                input: "",
                output: vec![
                    0xda, 0x39, 0xa3, 0xee,
                    0x5e, 0x6b, 0x4b, 0x0d,
                    0x32, 0x55, 0xbf, 0xef,
                    0x95, 0x60, 0x18, 0x90,
                    0xaf, 0xd8, 0x07, 0x09,
                ],
                output_str: "da39a3ee5e6b4b0d3255bfef95601890afd80709"
            },
            Test {
                input: "The quick brown fox jumps over the lazy dog",
                output: vec![
                    0x2f, 0xd4, 0xe1, 0xc6,
                    0x7a, 0x2d, 0x28, 0xfc,
                    0xed, 0x84, 0x9e, 0xe1,
                    0xbb, 0x76, 0xe7, 0x39,
                    0x1b, 0x93, 0xeb, 0x12,
                ],
                output_str: "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
            },
            Test {
                input: "The quick brown fox jumps over the lazy cog",
                output: vec![
                    0xde, 0x9f, 0x2c, 0x7f,
                    0xd2, 0x5e, 0x1b, 0x3a,
                    0xfa, 0xd3, 0xe8, 0x5a,
                    0x0b, 0xd1, 0x7d, 0x9b,
                    0x10, 0x0d, 0xb4, 0xb3,
                ],
                output_str: "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3",
            },
        ];

        for test in tests {
            // Hash through high-level API, check hex encoding/decoding
            let hash = Sha1Hash::hash(&test.input.as_bytes());
            assert_eq!(hash, Sha1Hash::from_hex(test.output_str).expect("parse hex"));
            assert_eq!(&hash[..], &test.output[..]);
            assert_eq!(&hash.to_hex(), &test.output_str);

            // Hash through engine, checking that we can input byte by byte
            let mut engine = Sha1Hash::engine();
            for ch in test.input.as_bytes() {
                engine.write(&[*ch]).expect("write to engine");
            }
            let manual_hash = Sha1Hash::from_engine(engine);
            assert_eq!(hash, manual_hash);
        }
    }
}

#[cfg(all(test, feature="unstable"))]
mod benches {
    use std::io::Write;
    use test::Bencher;

    use sha1::Sha1Hash;
    use Hash;

    #[bench]
    pub fn sha1_10(bh: & mut Bencher) {
        let mut engine = Sha1Hash::engine();
        let bytes = [1u8; 10];
        bh.iter( || {
            engine.write(&bytes).expect("write");
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    pub fn sha1_1k(bh: & mut Bencher) {
        let mut engine = Sha1Hash::engine();
        let bytes = [1u8; 1024];
        bh.iter( || {
            engine.write(&bytes).expect("write");
        });
        bh.bytes = bytes.len() as u64;
    }

    #[bench]
    pub fn sha1_64k(bh: & mut Bencher) {
        let mut engine = Sha1Hash::engine();
        let bytes = [1u8; 65536];
        bh.iter( || {
            engine.write(&bytes).expect("write");
        });
        bh.bytes = bytes.len() as u64;
    }

}
