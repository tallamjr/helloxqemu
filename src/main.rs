#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // This is where you would place your code.
    // For now, we'll just loop infinitely.
    loop {}
}
