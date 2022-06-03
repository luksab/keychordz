pub mod serial {
    use avr_device::interrupt::Mutex;
    use ufmt::uWrite;
    use core::cell::RefCell;

    pub type Usart = arduino_hal::hal::usart::Usart1<arduino_hal::DefaultClock>;
    pub static GLOBAL_SERIAL: Mutex<RefCell<Option<Usart>>> = Mutex::new(RefCell::new(None));

    pub fn init(serial: Usart) {
        avr_device::interrupt::free(|cs| {
            GLOBAL_SERIAL.borrow(&cs).replace(Some(serial));
        })
    }

    pub fn print_str(s: &str) {
        avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::global_print::serial::GLOBAL_SERIAL.borrow(&cs).borrow_mut() {
                serial.write_str(s).unwrap();
            } else {
                // Ok(());
            }
        })
    }

    pub fn print_buff(s: &'static [u8; 7]) {
        avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::global_print::serial::GLOBAL_SERIAL.borrow(&cs).borrow_mut() {
                for i in s {
                    serial.write_byte(*i);
                }
            } else {
                // Ok(());
            }
        })
    }

    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
                if let Some(serial) = &mut *crate::global_print::serial::GLOBAL_SERIAL.borrow(&cs).borrow_mut() {
                    ::ufmt::uwriteln!(serial, $($arg)*)
                } else {
                    Ok(())
                }
            })
        }
    }
}
