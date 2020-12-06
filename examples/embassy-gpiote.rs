//! Demo showing how to react to buttons using embassy async await support When
//! starts up, blue led is lit. Pushing the top button down will disable it.
//! Letting go will relight it.
//!
//! Also includes a panic handler. If your code panics the green led will be
//! stolen and lit.
//!
//! To program the dongle, enter the bootloader by pushing the right facing
//! button so the red led is blinking then run
//! Embassy requires nightly.
//! cargo +nightly run --example embassy-gpiote
//!
//! Pinout
//! BUTTON1 = SW1 = P1.6
//! RESET = SW2 = P0.18
//! LED0 (green) = P0.6
//! LED1 (red) = P0.8
//! LED1 (green) = P1.9
//! LED1 (blue) = P0.12
//! https://infocenter.nordicsemi.com/pdf/nRF52840_Dongle_User_Guide_v1.0.pdf

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_nrf::hal;

use embassy::executor::{task, Executor};
use embassy::util::Forever;
use embassy_nrf::gpiote::{Gpiote, PortInputPolarity};
use hal::gpio;
use hal::prelude::OutputPin;

static EXECUTOR: Forever<Executor> = Forever::new();

#[task]
async fn run() {
    let p = embassy_nrf::pac::Peripherals::take().unwrap();
    let port0 = gpio::p0::Parts::new(p.P0);
    let port1 = gpio::p1::Parts::new(p.P1);

    let mut led = port0.p0_12.into_push_pull_output(gpio::Level::Low);
    let g = Gpiote::new(p.GPIOTE);

    let pin1 = port1.p1_06.into_pullup_input().degrade();
    let button1 = async {
        loop {
            let _ = led.set_low();
            g.wait_port_input(&pin1, PortInputPolarity::Low).await;
            let _ = led.set_high();
            g.wait_port_input(&pin1, PortInputPolarity::High).await;
        }
    };

    button1.await;
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let executor = EXECUTOR.put(Executor::new(cortex_m::asm::sev));
    executor.spawn(run()).unwrap();

    loop {
        executor.run();
        cortex_m::asm::wfe();
    }
}

// steal peripherals and enable green led on panic
#[panic_handler]
fn panic(_p: &core::panic::PanicInfo) -> ! {
    let p = unsafe { embassy_nrf::pac::Peripherals::steal() };
    let port0 = gpio::p0::Parts::new(p.P0);
    let _led = port0.p0_06.into_push_pull_output(gpio::Level::Low);

    loop {
        cortex_m::asm::udf();
    }
}
