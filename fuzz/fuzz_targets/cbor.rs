
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bitcoin_hashes;
extern crate serde_cbor;

use bitcoin_hashes::Hmac;
use bitcoin_hashes::sha1::Sha1Hash;
use bitcoin_hashes::sha512::Sha512Hash;
use bitcoin_hashes::ripemd160::Ripemd160Hash;
use bitcoin_hashes::sha256d::Sha256dHash;

#[derive(Deserialize, Serialize)]
struct Hmacs {
    sha1: Hmac<Sha1Hash>,
    sha512: Hmac<Sha512Hash>,
}

#[derive(Deserialize, Serialize)]
struct Main {
    hmacs: Hmacs,
    ripemd: Ripemd160Hash,
    sha2d: Sha256dHash,
}

fn do_test(data: &[u8]) {
    if let Ok(m) = serde_cbor::from_slice::<Main>(data) {
        let vec = serde_cbor::to_vec(&m).unwrap();
        assert_eq!(data, &vec[..]);
    }
}

#[cfg(feature="honggfuzz")]
#[macro_use]
extern crate honggfuzz;

#[cfg(feature="honggfuzz")]
fn main() {
    loop {
        fuzz!(|d| { do_test(d) });
    }
}

#[cfg(test)]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'...b'F' => b |= c - b'A' + 10,
                b'a'...b'f' => b |= c - b'a' + 10,
                b'0'...b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("00000", &mut a);
        super::do_test(&a);
    }
}

