// ensure_no_std/src/main.rs
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use cortex_m_rt::entry;
use panic_halt as _;
use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
unsafe fn main() -> ! {
    let heap = [0u8; 1024];
    unsafe { ALLOCATOR.init((&heap) as *const u8 as usize, heap.len()) }

    loop {}
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!("OOM");
}