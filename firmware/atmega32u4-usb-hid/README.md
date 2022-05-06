# Atmega32u4 USB Keyboard Crate

This is a Rust crate for the Atmega32u4 microcontroller. It implements a CDC ACM USB serial device using the [Teensy AVR C library](https://www.pjrc.com/teensy/usb_keyboard.html).

It is adapted from the [atmega32u4-usb-serial](https://github.com/mogenson/atmega32u4-usb-serial) crate.

## To build

Use a nightly compiler version with AVR support by appending `+nightly` to each `cargo` command, or running `rustup override set nightly` once in the crate directory.

Run `cargo build`.

## To use

To upload the `echo.rs` example to a connected Arduino Leonardo board, run `cargo run --example echo`. Open the virtual serial port with a serial terminal and enter lowercase characters. They should be repeated back in uppercase.

Look at `src/lib.rs` and `examples/echo.rs` to see how to use the available USB keyboard methods.
