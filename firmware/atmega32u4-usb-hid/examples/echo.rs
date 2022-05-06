#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use avr_device::atmega32u4;
use atmega32u4_usb_hid::{UsbKeyboard, Key, Modifier};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = atmega32u4::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // #define DIRECT_PINS { { C6, D4, D7, E6 }, { B3, B2, B1, NO_PIN } }
    let mut usb = UsbKeyboard::new(dp.USB_DEVICE);
    usb.init(&dp.PLL);

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

    loop {
        if pins.d10.is_high() {
            UsbKeyboard::press_key(Key::A, Modifier::None).unwrap();
        }
        for key in Key::A as u8..=Key::Z as u8 {
            UsbKeyboard::press_keycode(key, Modifier::None).unwrap();
            delay_ms(100);
        }
    }
}
