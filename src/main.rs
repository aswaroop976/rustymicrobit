#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p: Peripherals = Peripherals::take().unwrap();
    p.P0.pin_cnf[21].write(|writable| writable.dir().output());
    p.P0.pin_cnf[28].write(|writable| writable.dir().output());

    let mut is_on: bool = false;
    loop {
        p.P0.out.write(|writable| writable.pin21().bit(is_on));
        for _ in 0..200_000 {
            nop();
        }
        is_on = !is_on;
    }
    // let mut x: usize = 0;
    // loop {
    //     x += 1;
    //     for _ in 0..x {
    //         nop();
    //     }
    // }
}
