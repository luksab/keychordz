#![feature(abi_avr_interrupt)]
#![feature(optimize_attribute)]
#![feature(generic_const_exprs)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

//! Keyboard firmware for Keychordz
//!
//! Flash using ```cargo run --release```

#[macro_use]
extern crate alloc;

mod allocator;
mod eeprom;
mod global_print;
mod key_handler;
mod key_prot;
mod key_state;
mod led;
mod millis;

use arduino_hal::delay_ms;
use atmega32u4_usb_hid::UsbKeyboard;
use avr_device::atmega32u4;
use key_handler::KeyHandler;
use key_handler::Layer;
use key_prot::KeyProt;
use led::*;

use core::panic::PanicInfo;

/// uncomment to enable debug prints
#[panic_handler]
#[allow(unused_variables)]
fn panic(info: &PanicInfo) -> ! {
    global_print::serial::print_str("panic!\n");
    // global_print::serial::print_buff(b"panic!\n");
    // let location = match info.location() {
    //     Some(loc) => (loc.file(), loc.line()),
    //     None => loop {},
    // };
    // let uloc = key_handler::UString(location.0.into());
    // println!("Crashed at {}:{}", uloc, location.1);
    // black_box(location.0);

    // if let Some(&s) = info.payload().downcast_ref::<&str>() {
    //     global_print::serial::print_str("payload\n");
    //     global_print::serial::print_str(s);
    // } else {
    //     global_print::serial::print_str("no payload\n");
    // }

    // if let Some(&l) = info.location() {
    //     println!("at {} {}:{}", l.file(), l.line(), l.column());
    // } else {
    //     println!("without location information");
    // }

    // if let Some(&args) = info.message() {
    //     if let Some(s) = args.as_str() {
    //         global_print::serial::print_str(s);
    //     } else {
    //         global_print::serial::print_str(&alloc::string::ToString::to_string(&args));
    //     }
    // } else {
    // }

    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = atmega32u4::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    global_print::serial::init(arduino_hal::default_serial!(dp, pins, 115200));

    let mut eeprom = eeprom::EEPROMHal::new(dp.EEPROM);

    let layer = Layer::default();
    eeprom.write_struct(0, &layer);

    millis::millis_init(dp.TC0);


    let layers = eeprom.read_struct(0);

    // println!("Layers: {:?}", layers);
    println!("Hello from Keychordz!");

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
            println!("Partner pulled d2 low");
            let d3 = pins.d3.into_output();
            delay_ms(10); // wait for the other side
            let d3 = d3.into_floating_input();
            break (false, pins.d2, d3);
        }
        if usb.usb_configured() {
            println!("USB initialized");
            let d2 = pins.d2.into_output();
            while pins.d3.is_high() {} // wait for the other side
            println!("Partner pulled d3 low");
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

    // let layers = eeprom.read_struct(0);

    // println!("Layers: {:?}", layers);

    let mut key_handler = KeyHandler::new(vec![layers]);

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
                    // println!("Wrote {:?}", &[keys_pressed]);
                }
                Err(e) => {
                    println!("write Error: {:?}", e);
                }
            }
        } else {
            // expecting key state, which is a u8, so one byte
            let mut buf = [0; 1];
            let bytes_read = match key_prot.read_blocking(&mut buf) {
                Ok(size) => size,
                Err(key_prot::Error::Overflow) => {
                    println!("Overflow");
                    buf.len() as u8
                }
                Err(e) => {
                    println!("read Error: {:?}", e);
                    continue;
                }
            };
            if bytes_read == 0 {
                println!("No bytes read");
                continue;
            }
            println!("{:?}", &buf);

            // update key state with the new keys
            if is_right {
                key_handler.update(buf[0], keys_pressed)
            } else {
                key_handler.update(keys_pressed, buf[0])
            };

            // println!("Ks: {:?}", &key_handler.should_trigger);

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
