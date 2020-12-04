//! Demo showing lighting the blue led.
//!
//! Also includes a panic handler. If your code panics the green led will be
//! stolen and lit.
//!
//! To program the dongle, enter the bootloader by pushing the right facing
//! button so the red led is blinking then run
//! cargo run
//!
//! Pinout
//! BUTTON1 = SW1 = P1.6
//! RESET = SW2 = P0.18
//! LED0 (green) = P0.6
//! LED1 (red) = P0.8
//! LED1 (green) = P1.9
//! LED1 (blue) = P0.12
//! https://infocenter.nordicsemi.com/pdf/nRF52840_Dongle_User_Guide_v1.0.pdf

#![no_main]
#![no_std]

use nrf52840_hal as hal;

use hal::gpio;
use hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let pins = gpio::p0::Parts::new(p.P0);

    let mut led = pins.p0_12.into_push_pull_output(gpio::Level::Low);

    let _ = led.set_low();

    loop {
        continue;
    }
}

// steal peripherals and enable green led on panic
#[panic_handler]
fn panic(_p: &core::panic::PanicInfo) -> ! {
    let p = unsafe { hal::pac::Peripherals::steal() };
    let port0 = gpio::p0::Parts::new(p.P0);
    let _led = port0.p0_06.into_push_pull_output(gpio::Level::Low);

    loop {
        cortex_m::asm::udf();
    }
}
