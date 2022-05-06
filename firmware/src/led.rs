use arduino_hal::port::{Pin, mode::Output};

pub fn write_to_led(pin: &mut Pin<Output>, array: &[u8]) {
    for val in array {
        for b in (0..8).rev() {
            let bit = (val >> b) & 0x1;

            if bit != 0 {
                // Send a ONE

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
            } else {
                // Send a ZERO

                pin.set_high();
                pin.set_low();
            }
        }
    }
}