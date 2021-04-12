#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

#[macro_use]
extern crate bitcoin_hashes;

extern crate alloc;
use panic_halt as _;

use self::alloc::string::ToString;
use self::alloc::vec;
use self::alloc::vec::Vec;
use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use bitcoin_hashes::core2::io::Write;
use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;

hash_newtype!(TestType, sha256::Hash, 32, doc = "test");

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mut engine = sha256::Hash::engine();
    engine.write_all(&[]).unwrap();
    let hash = sha256::Hash::from_engine(engine);
    hprintln!("hash {}", a).unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
