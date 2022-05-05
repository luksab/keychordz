use arduino_hal::{
    delay_ms, delay_us,
    hal::{
        delay,
        port::{PD0, PD1},
    },
    port::{
        mode::{Floating, Input, Output, PullUp},
        Pin,
    },
};
use ufmt::derive::uDebug;

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
    Other,
}

impl KeyProt {
    pub fn new(clk: Pin<Input<Floating>, PD0>, dta: Pin<Input<Floating>, PD1>) -> Self {
        Self {
            clk: Some(clk.into_pull_up_input()),
            dta: Some(dta.into_pull_up_input()),
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

        // wait another 10ms just to be sure
        delay_ms(10);

        // start write
        let mut dta = dta.into_output();
        for byte in data {
            // write byte
            for i in 0..8 {
                if *byte & (1 << i) != 0 {
                    dta.set_high();
                } else {
                    dta.set_low();
                }
                delay_us(500);
                clk.set_high();
                delay_ms(1);
                clk.set_low();
                delay_us(500);
            }
        }

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
        delay_ms(1);
        let dta = dta.into_pull_up_input();

        // read data
        let mut bytes_read = 0;
        // TODO: stop reading when transaction is complete
        for byte in buffer {
            for i in 0..8 {
                while clk.is_low() {}
                if dta.is_high() {
                    *byte |= 1 << i;
                } else {
                    *byte &= !(1 << i);
                }
                let mut clock_cycles = 0;
                while clk.is_high() {
                    clock_cycles += 1;
                    if clock_cycles > 100_000 {
                        self.clk = Some(clk.into_pull_up_input());
                        self.dta = Some(dta.into_pull_up_input());
                        return Err(Error::Other);
                    }
                }
                bytes_read += 1;
            }
        }

        self.clk = Some(clk.into_pull_up_input());
        self.dta = Some(dta.into_pull_up_input());

        Ok(bytes_read)
    }
}
