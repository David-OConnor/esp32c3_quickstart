//! See https://github.com/esp-rs/esp-templatehttps://github.com/esp-rs/esp-template
//! cargo generate esp-rs/esp-template #no_std

#![no_main]
#![no_std]


use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};

// use esp_wifi::{initialize, EspWifiInitFor};

use hal::{systimer::SystemTimer, Rng};

// use defmt_rtt as _;
// use panic_probe as _;
// use defmt::println;

// use esp_backtrace as _;
// use esp_println::println;

mod init;
mod setup;
mod system_status;

pub struct Config {}

// #[rtic::app(device = pac, peripherals = false)]
// mod app {
//     use super::*;
//
//     #[shared]
//     pub struct Shared {
//         pub config: Config,
//         pub system_status: system_status::SystemStatus,
//     }
//
//     #[local]
//     pub struct Local {}
//
//     #[init]
//     fn init(cx: init::Context) -> (Shared, Local) {
//         crate::init::run(cx)
//     }
//
//     #[idle(shared = [], local = [])]
//     /// In this function, we perform setup code that must occur with interrupts enabled.
//     fn idle(_cx: idle::Context) -> ! {
//         loop {
//             asm::nop();
//         }
//     }
// }

#[entry]
fn main() -> ! {
    init::run();

    loop {
        // todo?
        // // low_power::sleep_now();
        // cortex_m::asm::nop();
    }
}

// // same panicking *behavior* as `panic-probe` but doesn't print a panic message
// // this prevents the panic message being printed *twice* when `defmt::panic` is invoked
// #[defmt::panic_handler]
// fn panic() -> ! {
//     cortex_m::asm::udf()
// }
