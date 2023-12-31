# ESP32-C3 quickstart

No-std, no Async.

# Quickstart
- [Install Rust](https://www.rust-lang.org/tools/install).
- Install the compilation target for your MCU. Eg run `rustup target add riscv32imc-unknown-none-elf `.
- Install flash and debug tools: `cargo install flip-link`, `cargo install probe-rs --features cli`.
- Connect your device. Run `cargo run --release` to compile and flash.
