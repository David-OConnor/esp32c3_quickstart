//! This module contains initialization code, run once at program start.

// use esp_backtrace as _;
// use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};

// use esp_wifi::{initialize, EspWifiInitFor};

use hal::{systimer::SystemTimer, Rng};

use crate::setup;

pub fn run() {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // println!("Hello world!");
    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    // let _init = initialize(
    //     EspWifiInitFor::Wifi,
    //     timer,
    //     Rng::new(peripherals.RNG),
    //     system.radio_clock_control,
    //     &clocks,
    // )
    //     .unwrap();

    loop {
        // println!("Loop...");
        delay.delay_ms(500u32);
    }
}
