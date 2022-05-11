#![no_std]
#![feature(abi_avr_interrupt)]
#![allow(dead_code)]

use avr_device::atmega32u4::{PLL, USB_DEVICE};
pub use defines::*;

extern "C" {
    /* general */
    fn usb_init();
    fn usb_configured() -> u8;

    /* USB */
    fn usb_keyboard_press(key: u8, modifier: u8) -> i8;
    fn usb_keyboard_send() -> i8;
    static mut keyboard_keys: [u8; 6];
    static mut keyboard_modifier_keys: u8;
    static keyboard_leds: u8;
}

pub struct UsbKeyboard {
    usb: USB_DEVICE,
}

impl UsbKeyboard {
    pub fn new(usb: USB_DEVICE) -> Self {
        Self { usb }
    }

    /// Blocking USB initialization
    /// 
    /// For non-blocking initialization, see `init_async`
    pub fn init(&mut self, pll: &PLL) {
        self.usb.uhwcon.write(|w| w.uvrege().set_bit());
        self.usb
            .usbcon
            .write(|w| w.usbe().set_bit().frzclk().set_bit());
        pll.pllcsr.write(|w| w.pindiv().set_bit().plle().set_bit());
        while pll.pllcsr.read().plock().bit_is_clear() {}
        self.usb
            .usbcon
            .write(|w| w.usbe().set_bit().frzclk().clear_bit().otgpade().set_bit());
        self.usb.udcon.write(|w| w.detach().clear_bit());
        self.usb
            .udien
            .write(|w| w.eorste().set_bit().sofe().set_bit());

        unsafe {
            usb_init();
            avr_device::interrupt::enable();
            while usb_configured() == 0 {}
        }
    }

    /// Non-blocking USB initialization
    /// 
    /// For blocking initialization, see `init`
    /// 
    /// Check if the USB is configured with `usb_configured()`
    pub fn init_async(&mut self, pll: &PLL) {
        self.usb.uhwcon.write(|w| w.uvrege().set_bit());
        self.usb
            .usbcon
            .write(|w| w.usbe().set_bit().frzclk().set_bit());
        pll.pllcsr.write(|w| w.pindiv().set_bit().plle().set_bit());
        while pll.pllcsr.read().plock().bit_is_clear() {}
        self.usb
            .usbcon
            .write(|w| w.usbe().set_bit().frzclk().clear_bit().otgpade().set_bit());
        self.usb.udcon.write(|w| w.detach().clear_bit());
        self.usb
            .udien
            .write(|w| w.eorste().set_bit().sofe().set_bit());

        unsafe {
            usb_init();
            avr_device::interrupt::enable();
        }
    }

    pub fn usb_configured(&self) -> bool {
        unsafe { usb_configured() != 0 }
    }

    pub fn press_key(key: Key, modifier: Modifier) -> Result<(), ()> {
        unsafe {
            keyboard_keys[1..].fill(0); // clear all keys, except the first
            match usb_keyboard_press(key as u8, modifier as u8) {
                -1 => Err(()),
                -2 => Err(()),
                _ => Ok(()),
            }
        }
    }

    pub fn press_keycode(key: u8, modifier: Modifier) -> Result<(), ()> {
        unsafe {
            keyboard_keys[1..].fill(0); // clear all keys, except the first
            match usb_keyboard_press(key, modifier as u8) {
                -1 => Err(()),
                -2 => Err(()),
                _ => Ok(()),
            }
        }
    }

    pub fn press_keys(keys: &[Key], modifier: Modifier) -> Result<(), ()> {
        for (i, key) in keys.into_iter().enumerate() {
            unsafe {
                keyboard_keys[i] = *key as u8;
            }
        }
        unsafe {
            keyboard_modifier_keys = modifier as u8;
            match usb_keyboard_send() {
                -1 => Err(()),
                -2 => Err(()),
                _ => Ok(()),
            }
        }
    }
}

// impl ufmt::uWrite for UsbKeyboard {
//     type Error = ();

//     fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
//         if unsafe { usb_serial_write(s.as_ptr(), s.len() as u16) == 0 } {
//             Ok(())
//         } else {
//             Err(())
//         }
//     }
// }

// / Handlers for the `USB_GEN` and `USB_COM` interrupts.
// /
// / When the `rt` feature is enabled (which it is by default), this crate defines ISRs for
// / `USB_GEN` and `USB_COM` which are necessary for proper operation.  This will also pull in
// / `avr-device/rt`.
// /
// / If you need to define these ISRs yourself, you can disable the `rt` feature.  You then need to
// / manually call [`isr::usb_gen`] and [`isr::usb_com`] in your implementation.  For example:
// /
// / ```no_run
// / mod usb_isr {
// /     use atmega32u4_usb_serial::isr;
// /
// /     #[avr_device::interrupt(atmega32u4)]
// /     unsafe fn USB_GEN() {
// /         isr::usb_gen()
// /     }
// /
// /     #[avr_device::interrupt(atmega32u4)]
// /     unsafe fn USB_COM() {
// /         isr::usb_com()
// /     }
// / }
// / ```
pub mod isr {
    //     use super::*;

    //     /// ISR implementation for the `USB_GEN` interrupt.
    //     #[inline(always)]
    //     pub unsafe fn usb_gen() {
    //         avr_device::interrupt::free(|_| usb_gen_handler());
    //     }

    //     /// ISR implementation for the `USB_COM` interrupt.
    //     #[inline(always)]
    //     pub unsafe fn usb_com() {
    //         avr_device::interrupt::free(|_| usb_com_handler());
    //     }

    //     #[cfg(feature = "rt")]
    //     mod rt {
    //         #[avr_device::interrupt(atmega32u4)]
    //         unsafe fn USB_GEN() {
    //             super::usb_gen()
    //         }

    //         #[avr_device::interrupt(atmega32u4)]
    //         unsafe fn USB_COM() {
    //             super::usb_com()
    //         }
    //     }
}
