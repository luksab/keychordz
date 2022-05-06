use arduino_hal::{
    delay_us,
    hal::port::{PD0, PD1},
    port::{
        mode::{Floating, Input, Output, PullUp},
        Pin,
    },
};
use ufmt::derive::uDebug;

const END_MARKER: u8 = 0b0111_1110;
const TIMEOUT: u16 = 1_000;

/// Struct for Protocol of the Keyboard sides
/// communicating with each other
pub struct KeyProt {
    clk: Option<Pin<Input<PullUp>, PD0>>,
    dta: Option<Pin<Input<PullUp>, PD1>>,
}

#[derive(Debug, Clone, Copy, uDebug)]
pub enum Error {
    PinsBusy,
    AlreadyWriting,
    Overflow,
    IncorrectEndMarker,
    Timeout,
    Other,
}

impl KeyProt {
    pub fn new(clk: Pin<Input<Floating>, PD0>, dta: Pin<Input<Floating>, PD1>) -> Self {
        Self {
            clk: Some(clk.into_pull_up_input()),
            dta: Some(dta.into_pull_up_input()),
        }
    }

    #[inline(always)]
    fn _write_bit(clk: &mut Pin<Output, PD0>, dta: &mut Pin<Output, PD1>, bit: bool) {
        delay_us(5);
        clk.set_low();
        if bit {
            dta.set_high();
        } else {
            dta.set_low();
        }
        delay_us(5);
        clk.set_high();
    }

    /// write after write has started
    ///
    /// with bit stuffing
    #[inline(always)]
    fn _write(clk: &mut Pin<Output, PD0>, dta: &mut Pin<Output, PD1>, data: &[u8]) {
        let mut consecutive_ones = 0;
        for byte in data {
            // write byte
            for i in 0..8 {
                if *byte & (1 << i) != 0 {
                    consecutive_ones += 1;
                    if consecutive_ones == 5 {
                        // encode 0
                        Self::_write_bit(clk, dta, false);
                        consecutive_ones = 0;
                    }
                    Self::_write_bit(clk, dta, true);
                } else {
                    consecutive_ones = 0;
                    Self::_write_bit(clk, dta, false);
                }
            }
        }
        // end marker
        for i in 0..8 {
            if END_MARKER & (1 << i) != 0 {
                Self::_write_bit(clk, dta, true);
            } else {
                Self::_write_bit(clk, dta, false);
            }
        }
    }

    /// read after read has started
    ///
    /// with bit unstuffing
    #[inline(always)]
    fn _read(
        buffer: &mut [u8],
        clk: &Pin<Input<PullUp>, PD0>,
        dta: &Pin<Input<PullUp>, PD1>,
    ) -> Result<u8, Error> {
        let mut consecutive_ones = 0;
        let mut overflow = false;
        for byte in 0u8.. {
            let mut byte_read = 0u8;
            let mut bit = 0u8;
            loop {
                while clk.is_low() {}
                if dta.is_high() {
                    consecutive_ones += 1;
                    if consecutive_ones == 5 {
                        // ignore the next 0
                        let mut clock_cycles = 0;
                        while clk.is_high() {
                            clock_cycles += 1;
                            if clock_cycles > TIMEOUT {
                                return Err(Error::Timeout);
                            }
                        }
                        while clk.is_low() {}
                        if dta.is_high() {
                            // 6 consecutive 1s
                            while clk.is_low() {}
                            let mut clock_cycles = 0;
                            while clk.is_high() {
                                clock_cycles += 1;
                                if clock_cycles > TIMEOUT {
                                    return Err(Error::Timeout);
                                }
                            }
                            if dta.is_high() {
                                // last bit should be a 0
                                return Err(Error::IncorrectEndMarker);
                            }
                            return Ok(byte);
                        }
                        // else: ignore the 0
                        consecutive_ones = 0;
                    }
                    byte_read |= 1 << bit;
                } else {
                    consecutive_ones = 0;
                    // byte_read |= 0 << bit;
                }

                let mut clock_cycles = 0;
                while clk.is_high() {
                    clock_cycles += 1;
                    if clock_cycles > TIMEOUT {
                        return Err(Error::Timeout);
                    }
                }
                bit += 1;
                if bit == 8 {
                    break;
                }
            }
            if byte > buffer.len() as u8 {
                overflow = true;
            } else {
                buffer[byte as usize] = byte_read;
            }
        }
        if overflow {
            Err(Error::Overflow)
        } else {
            Ok(buffer.len() as u8)
        }
    }

    pub fn write_blocking(&mut self, data: &[u8]) -> Result<(), Error> {
        // take clk and dta pins
        let clk = match self.clk.take() {
            Some(clk) => clk,
            None => return Err(Error::PinsBusy),
        };
        let dta = match self.dta.take() {
            Some(dta) => dta,
            None => {
                self.clk = Some(clk.into_pull_up_input());
                return Err(Error::PinsBusy);
            }
        };
        // check that clk is high, meaning no transaction is in progress
        if clk.is_low() {
            self.clk = Some(clk.into_pull_up_input());
            self.dta = Some(dta.into_pull_up_input());
            return Err(Error::AlreadyWriting);
        }
        // pull low to indicate pending write
        let mut clk = clk.into_output();

        // wait for other side to be ready
        while dta.is_high() {}

        // wait another 10us just to be sure
        delay_us(100);

        // start write
        let mut dta = dta.into_output();
        Self::_write(&mut clk, &mut dta, data);

        self.clk = Some(clk.into_pull_up_input());
        self.dta = Some(dta.into_pull_up_input());
        Ok(())
    }

    /// read up to buffer.len() bytes, returning the number of bytes read
    pub fn read_blocking(&mut self, buffer: &mut [u8]) -> Result<u8, Error> {
        // take clk and dta pins
        let clk = match self.clk.take() {
            Some(clk) => clk,
            None => return Err(Error::PinsBusy),
        };
        let dta = match self.dta.take() {
            Some(dta) => dta,
            None => {
                self.clk = Some(clk.into_pull_up_input());
                return Err(Error::PinsBusy);
            }
        };
        // wait for write
        while clk.is_high() {}

        // confirm ready
        let dta = dta.into_output();
        // delay_ms(1);
        delay_us(100);
        let dta = dta.into_pull_up_input();

        // read data
        let bytes_read = Self::_read(buffer, &clk, &dta);

        // delay_ms(1);

        self.clk = Some(clk.into_pull_up_input());
        self.dta = Some(dta.into_pull_up_input());

        bytes_read
    }
}
