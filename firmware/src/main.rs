#![feature(llvm_asm)]
#![feature(abi_avr_interrupt)]
#![feature(optimize_attribute)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

//! Keyboard firmware for Keychordz
//!
//! Flash using ```cargo run --release```

#[macro_use]
extern crate alloc;

mod allocator;
mod key_prot;
mod key_state;
mod key_handler;
mod led;
mod millis;
mod eeprom;

use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
use atmega32u4_usb_hid::UsbKeyboard;
use avr_device::atmega32u4;
use key_handler::KeyHandler;
use key_prot::KeyProt;
use led::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = atmega32u4::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    millis::millis_init(dp.TC0);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    ufmt::uwriteln!(&mut serial, "Hello!").void_unwrap();

    // #define DIRECT_PINS { { C6, D4, D7, E6 }, { B3, B2, B1, NO_PIN } }
    let mut usb = UsbKeyboard::new(dp.USB_DEVICE);
    usb.init_async(&dp.PLL);

    // protocol:
    // USB attached device pulls d2 low,
    // other device pulls d3 low,
    // both devices start i2c
    let (is_usb, d2, d3) = loop {
        // detect if other device is connected to USB
        if pins.d2.is_low() {
            ufmt::uwriteln!(&mut serial, "Partner pulled d2 low").void_unwrap();
            let d3 = pins.d3.into_output();
            delay_ms(10); // wait for the other side
            let d3 = d3.into_floating_input();
            break (false, pins.d2, d3);
        }
        if usb.usb_configured() {
            ufmt::uwriteln!(&mut serial, "USB initialized").void_unwrap();
            let d2 = pins.d2.into_output();
            while pins.d3.is_high() {} // wait for the other side
            ufmt::uwriteln!(&mut serial, "Partner pulled d3 low").void_unwrap();
            let d2 = d2.into_floating_input();
            break (true, d2, pins.d3);
        }
    };

    let side_pin = pins.d8.into_pull_up_input();
    delay_ms(10);

    let is_right = side_pin.is_low();

    let keys = [
        pins.d15.into_pull_up_input().downgrade(),
        pins.d14.into_pull_up_input().downgrade(),
        pins.d16.into_pull_up_input().downgrade(),
        pins.d7.into_pull_up_input().downgrade(),
        pins.d6.into_pull_up_input().downgrade(),
        pins.d4.into_pull_up_input().downgrade(),
        pins.d5.into_pull_up_input().downgrade(),
    ];

    let mut key_handler = KeyHandler::new();

    let mut key_prot = KeyProt::new(d3, d2);

    // let mut led_pin = pins.d9.into_output().downgrade();
    let mut led = Leds::<7>::new(pins.d9.into_output());

    loop {
        let mut keys_pressed = 0u8;
        for (i, key) in keys.iter().enumerate() {
            if key.is_low() {
                keys_pressed |= 1 << i;
            }
        }

        // switch code flow depending on USB state
        if !is_usb {
            match key_prot.write_blocking(&[keys_pressed]) {
                Ok(_) => {
                    ufmt::uwriteln!(&mut serial, "Wrote {:?}", &[keys_pressed]).void_unwrap();
                }
                Err(e) => {
                    ufmt::uwriteln!(&mut serial, "write Error: {:?}", e).void_unwrap();
                }
            }
        } else {
            // expecting key state, which is a u8, so one byte
            let mut buf = [0; 1];
            let bytes_read = match key_prot.read_blocking(&mut buf) {
                Ok(size) => size,
                Err(key_prot::Error::Overflow) => {
                    ufmt::uwriteln!(&mut serial, "Overflow").void_unwrap();
                    buf.len() as u8
                }
                Err(e) => {
                    ufmt::uwriteln!(&mut serial, "read Error: {:?}", e).void_unwrap();
                    continue;
                }
            };
            if bytes_read == 0 {
                ufmt::uwriteln!(&mut serial, "No bytes read").void_unwrap();
                continue;
            }
            ufmt::uwriteln!(&mut serial, "{:?}", &buf).void_unwrap();

            // update key state with the new keys
            if is_right {
                key_handler.update(buf[0], keys_pressed)
            } else {
                key_handler.update(keys_pressed, buf[0])
            };

            ufmt::uwriteln!(&mut serial, "Ks: {:?}", &key_handler.should_trigger).void_unwrap();

            // if keys_hit == 1 {
            //     led.brightness = led.brightness.saturating_add(10);
            // }
            // if keys_hit == 2 {
            //     led.brightness = led.brightness.saturating_sub(10);
            // }
        }

        led.draw();
    }
}
