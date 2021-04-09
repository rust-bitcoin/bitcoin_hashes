#![no_std]
#![no_main]

#[macro_use]
extern crate bitcoin_hashes;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;

hash_newtype!(TestType, sha256::Hash, 32, doc="test");

#[entry]
fn main() -> ! {
    hprintln!("Hello world!").unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        // your code goes here
    }
}
