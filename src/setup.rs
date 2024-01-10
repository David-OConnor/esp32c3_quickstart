//! This module contains hardware configuration setup, eg GPIO, DMA, and IO.

use hal::{
    clock::{ClockControl, Clocks},
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    uart::{
        config::{Config, DataBits, Parity, StopBits},
        TxRxPins,
    },
    Uart, IO,
};

const BAUD: u32 = 9_600;

pub fn setup_pins() {}

pub fn setup(clocks: &Clocks) {
    let peripherals = unsafe { Peripherals::steal() };

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio1.into_push_pull_output(),
        io.pins.gpio2.into_floating_input(),
    );

    let uart_config = Config {
        baudrate: BAUD,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
    };

    let mut uart = Uart::new_with_config(peripherals.UART1, uart_config, Some(pins), &clocks);
}

pub fn setup_dma() {
}
