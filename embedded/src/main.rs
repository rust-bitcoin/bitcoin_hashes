#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

#[macro_use]
extern crate bitcoin_hashes;

extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use bitcoin_hashes::{sha256, Hash, HashEngine};
use core::alloc::Layout;
use core::str::FromStr;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;

hash_newtype!(TestType, sha256::Hash, 32, doc = "test");

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mut engine = TestType::engine();
    engine.input(b"abc");
    let hash = TestType::from_engine(engine);

    let hash_check =
        TestType::from_str("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
            .unwrap();
    hprintln!("hash:{} hash_check:{}", hash, hash_check).unwrap();
    if hash == hash_check {
        debug::exit(debug::EXIT_SUCCESS);
    } else {
        debug::exit(debug::EXIT_FAILURE);
    }

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
