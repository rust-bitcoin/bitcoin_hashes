#![no_std]
#![no_main]

#[macro_use]
extern crate bitcoin_hashes;

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;

hash_newtype!(TestType, sha256::Hash, 32, doc="test");

#[entry]
fn main() -> ! {
    hprintln!("Hello world!").unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
