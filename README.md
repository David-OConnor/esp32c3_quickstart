# ESP32-C3 quickstart


# Quickstart
- [Install Rust](https://www.rust-lang.org/tools/install).
- Install the compilation target for your MCU. Eg run `rustup target add riscv32imc-unknown-none-elf`.
- `rustup toolchain install nightly --component rust-src`

[//]: # (- Install flash and debug tools: `cargo install flip-link`, `cargo install probe-rs --features cli`.)
- Connect your device using JTAG USB. Run `cargo run --release` to compile and flash. (Shortcut: `cargo rr`)
