//! See https://github.com/esp-rs/esp-templatehttps://github.com/esp-rs/esp-template
//! cargo generate esp-rs/esp-template #no_std

#![no_main]
#![no_std]

// use defmt_rtt as _;
// use defmt::println;
use esp_backtrace as _; // Required for the panic handler.
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
// use esp_wifi::{initialize, EspWifiInitFor};
use hal::{systimer::SystemTimer, Rng};

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
    // println!("Pre-init");
    init::run();

    println!("Post-init");

    loop {
        // todo?
        // // low_power::sleep_now();
        // cortex_m::asm::nop();
    }
}
