#![feature(llvm_asm)]
#![no_std]
#![no_main]

mod key_prot;

use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
use atmega32u4_usb_hid::UsbKeyboard;
use avr_device::atmega32u4;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = atmega32u4::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
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

    delay_ms(10);

    // pro micro pins:
    // pins.d2 = D1
    // pins.d3 = D0
    // pins.d4 = D4
    // pins.d5 = C6
    // pins.d6 = D7
    // pins.d7 = E6
    // pins.d8 = B4
    // pins.d9 = B5
    // pins.d10 = B6
    // pins.d14 = B2
    // pins.d15 = B1
    // pins.d16 = B3

    // pins.d15 = B1
    // pins.d14 = B2
    // pins.d16 = B3
    // pins.d8 = B4
    // pins.d9 = B5
    // pins.d10 = B6
    // pins.d5 = C6
    // pins.d3 = D0
    // pins.d2 = D1
    // pins.d4 = D4
    // pins.d6 = D7
    // pins.d7 = E6

    // let mut led_usb = pins.led_tx.into_output();
    // let mut led_i2c = pins.led_rx.into_output();

    let keys = [
        pins.d5.into_pull_up_input().downgrade(),
        pins.d4.into_pull_up_input().downgrade(),
        pins.d6.into_pull_up_input().downgrade(),
        pins.d7.into_pull_up_input().downgrade(),
        pins.d16.into_pull_up_input().downgrade(),
        pins.d14.into_pull_up_input().downgrade(),
        pins.d15.into_pull_up_input().downgrade(),
    ];

    let mut key_prot = key_prot::KeyProt::new(d3, d2);

    loop {
        let mut any_key_pressed = false;
        let mut keys_pressed = 0u8;
        for (i, key) in keys.iter().enumerate() {
            if key.is_low() {
                any_key_pressed = true;
                keys_pressed |= 1 << i;
            }
        }
        // led.set_high();
        if !is_usb {
            ufmt::uwriteln!(&mut serial, "writing {}", keys_pressed).void_unwrap();
            match key_prot.write_blocking(&[keys_pressed]) {
                Ok(_) => {
                    ufmt::uwriteln!(&mut serial, "Wrote {:?}", &[keys_pressed]).void_unwrap();
                }
                Err(e) => {
                    ufmt::uwriteln!(&mut serial, "write Error: {:?}", e).void_unwrap();
                    // led_i2c.set_high();
                    // delay_ms(100);
                    // led_i2c.set_low();
                    // delay_ms(100);
                }
            }
        } else {
            let mut buf = [0; 1];
            ufmt::uwriteln!(&mut serial, "reading").void_unwrap();
            let bytes_read = match key_prot.read_blocking(&mut buf) {
                Ok(size) => {size}
                Err(key_prot::Error::Overflow) => {
                    ufmt::uwriteln!(&mut serial, "Overflow").void_unwrap();
                    buf.len() as u8
                }
                Err(e) => {
                    ufmt::uwriteln!(&mut serial, "read Error: {:?}", e).void_unwrap();
                    continue;
                    // led_usb.set_high();
                    // delay_ms(100);
                    // led_usb.set_low();
                    // delay_ms(100);
                }
            };
            ufmt::uwriteln!(&mut serial, "read {:?}", &buf).void_unwrap();
            // any_key_pressed |= i2c_buffer[0] != 0;
            // if i2c_buffer[0] != 0 {
            //     led_i2c.set_high();
            // } else {
            //     led_i2c.set_low();
            // }
        }
        if any_key_pressed {
            // led_usb.set_high();
            ufmt::uwriteln!(&mut serial, "Key pressed").void_unwrap();
        } else {
            // led_usb.set_low();
        }
    }
}
