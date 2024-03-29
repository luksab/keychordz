use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};
pub use smart_leds::hsv::hsv2rgb;
use smart_leds::hsv::Hsv;

use crate::millis::millis;

/// Led modes
enum Modes {
    HueWaves,
}

/// Led struct owning the Pin the LEDs are connected to
///
/// Currently assumes Pin `PB5` and WS2812b LEDs
pub struct Leds<const N: usize>
where
    [u8; 3 * N]: Sized,
{
    pin: Pin<Output, PB5>,
    state: Modes,
    /// buffer containing the data to be sent on the next draw call
    /// three bytes per LED
    buffer: [u8; 3 * N],
    /// brightess will be gamma corrected
    pub brightness: u8,
}

impl<const N: usize> Leds<N>
where
    [u8; 3 * N]: Sized,
{
    /// construct new Led struct owning pin `PB5`
    pub fn new(pin: Pin<Output, PB5>) -> Self {
        Self {
            pin,
            state: Modes::HueWaves,
            buffer: [0; 3 * N],
            brightness: 50,
        }
    }

    /// update the buffer and send new state to LEDs
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

    /// write a zero to ws2812 LED strip
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
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
    }

    /// write a one to ws2812 LED strip
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
        pin.set_low();
        avr_device::asm::nop();
        avr_device::asm::nop();
        avr_device::asm::nop();
    }

    #[inline(always)]
    fn write_bit(pin: &mut Pin<Output, PB5>, val: u8, bit: u32) {
        if val & (1 << bit) != 0 {
            Self::write_one(pin);
        } else {
            Self::write_zero(pin);
        }
    }

    /// write buffer to LEDs
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
                // inline reverse loop for performance
                Self::write_bit(&mut self.pin, *val, 7);
                Self::write_bit(&mut self.pin, *val, 6);
                Self::write_bit(&mut self.pin, *val, 5);
                Self::write_bit(&mut self.pin, *val, 4);
                Self::write_bit(&mut self.pin, *val, 3);
                Self::write_bit(&mut self.pin, *val, 2);
                Self::write_bit(&mut self.pin, *val, 1);
                Self::write_bit(&mut self.pin, *val, 0);
            }
        });
    }
}
