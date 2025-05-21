#![no_std]
#![no_main]
#![deny(unsafe_code)]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;
    loop {}
}
