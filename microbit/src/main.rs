#![no_main]
#![no_std]
#![allow(unused)]
#![deny(unsafe_code)]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::*, hal::timer::Timer};
use panic_halt as _;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut current_display = [[0; 5]; 5];
    let mut flip = true;
    loop {
        if flip {
            current_display[0][0] = 1;
            display.show(&mut timer, current_display, 1000)
        } else {
            current_display[0][0] = 0;
            display.show(&mut timer, current_display, 1000)
        }
        flip = !flip;
        timer.delay(1000u32);
        //timer.delay_ms(1000u32);
        rprintln!("1 sec has passed");

        // infinite loop; just so we don't leave this stack frame
    }
}
