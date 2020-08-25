//! https://infocenter.nordicsemi.com/pdf/nRF52840_Dongle_User_Guide_v1.0.pdf
//! BUTTON1 = SW1 = P1.6
//! RESET = SW2 = P0.18
//! LED0 (green) = P0.6
//! LED1 (red) = P0.8
//! LED1 (green) = P1.9
//! LED1 (blue) = P0.12

#![no_main]
#![no_std]

use nrf52840_hal as hal;
use panic_halt as _;

use hal::gpio::p0;
use hal::gpio::Level;
use hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let pins = p0::Parts::new(p.P0);

    let mut led = pins.p0_12.into_push_pull_output(Level::Low);

    let _ = led.set_low();

    loop {
        continue;
    }
}
