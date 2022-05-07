use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};
pub use smart_leds::hsv::hsv2rgb;
use smart_leds::hsv::Hsv;

use crate::millis::millis;

enum Modes {
    HueWaves,
}

pub struct Leds<const N: usize>
where
    [u8; 3 * N]: Sized,
{
    pin: Pin<Output, PB5>,
    state: Modes,
    buffer: [u8; 3 * N],
    pub brightness: u8,
}

impl<const N: usize> Leds<N>
where
    [u8; 3 * N]: Sized,
{
    pub fn new(pin: Pin<Output, PB5>) -> Self {
        Self {
            pin,
            state: Modes::HueWaves,
            buffer: [0; 3 * N],
            brightness: 255,
        }
    }

    pub fn draw(&mut self) {
        match self.state {
            Modes::HueWaves => {
                for led in 0..N {
                    let offset = millis() + led as u32 * 100;
                    let hue = (offset / 8) % 255;
                    let hsv = Hsv {
                        hue: hue as u8,
                        sat: 255,
                        val: 255,
                    };
                    let rgb = hsv2rgb(hsv);
                    self.buffer[3 * led] = rgb.r;
                    self.buffer[3 * led + 1] = rgb.g;
                    self.buffer[3 * led + 2] = rgb.b;
                }
                self.write_to_led();
            }
        }
    }

    #[inline(always)]
    fn write_zero(pin: &mut Pin<Output, PB5>) {
        pin.set_high();
        avr_device::asm::nop();
        pin.set_low();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
    }

    #[inline(always)]
    fn write_one(pin: &mut Pin<Output, PB5>) {
        pin.set_high();
        // Wait exact number of cycles specified
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        pin.set_low();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
    }

    #[optimize(speed)]
    pub fn write_to_led(&mut self) {
        // brightness with gamma correction
        let brightness = self.brightness as f32 / 255.0;
        let brightness = (brightness * brightness * 255.0) as u8;
        self.buffer.iter_mut().for_each(|byte| {
            *byte = ((*byte as u16 * brightness as u16) / 255) as u8;
        });
        avr_device::interrupt::free(|_cs| {
            for val in self.buffer.iter() {
                for b in (0..8).rev() {
                    let bit = (val >> b) & 0x1;

                    if bit != 0 {
                        // Send a ONE

                        Self::write_one(&mut self.pin);
                    } else {
                        // Send a ZERO

                        Self::write_zero(&mut self.pin);
                    }
                }
            }
        });
    }
}

// #[optimize(speed)]
// pub fn write_to_led(pin: &mut Pin<Output>, array: &[u8]) {
//     for val in array {
//         for b in (0..8) {
//             let bit = (val >> b) & 0x1;

//             if bit != 0 {
//                 // Send a ONE

//                 pin.set_high();
//                 // unsafe {llvm_asm!("PORTC |= 0b00000001;")};
//                 // Wait exact number of cycles specified
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 avr_device::asm::nop();
//                 pin.set_low();
//             } else {
//                 // Send a ZERO

//                 pin.set_high();
//                 pin.set_low();
//             }
//         }
//     }
// }
