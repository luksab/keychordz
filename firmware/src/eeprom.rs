use core::mem::size_of;
use postcard::{from_bytes, to_vec};

use arduino_hal::pac::EEPROM;
use serde::{de::DeserializeOwned, Serialize};
pub struct EEPROMHal {
    eeprom_registers: EEPROM,
}

const MAX_STRUCT_SIZE: usize = 64;

impl EEPROMHal {
    pub fn new(eeprom_registers: EEPROM) -> EEPROMHal {
        EEPROMHal { eeprom_registers }
    }

    /// read byte from address
    pub fn read_byte(&mut self, address: usize) -> u8 {
        // wait for all writes to finish
        while self.eeprom_registers.eecr.read().eepe().bit_is_set() {}

        self.eeprom_registers
            .eear
            .write(|w| unsafe { w.bits(address as u16) });

        // Start read transaction
        self.eeprom_registers.eecr.write(|w| w.eere().set_bit());

        // Read the byte
        self.eeprom_registers.eedr.read().bits()
    }

    pub fn write_byte(&mut self, address: usize, data: u8) {
        // Wait for all writes to finish
        while self.eeprom_registers.eecr.read().eepe().bit_is_set() {}

        // Set address
        self.eeprom_registers
            .eear
            .write(|w| unsafe { w.bits(address as u16) });

        // Set data
        self.eeprom_registers
            .eedr
            .write(|w| unsafe { w.bits(data) });

        // Start write transaction
        self.eeprom_registers
            .eecr
            .modify(|_, w| w.eempe().set_bit());
        // Write
        self.eeprom_registers.eecr.modify(|_, w| w.eepe().set_bit());
    }

    pub fn read_buffer(&mut self, address: usize, buffer: &mut [u8]) {
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = self.read_byte(address + i);
        }
    }

    pub fn write_buffer(&mut self, address: usize, buffer: &[u8]) {
        for (i, byte) in buffer.iter().enumerate() {
            self.write_byte(address + i, *byte);
        }
    }

    pub fn write_buffer_with_len(&mut self, address: usize, buffer: &[u8], len: u8) {
        self.write_byte(address, len);
        self.write_buffer(address + 1, buffer);
    }

    pub fn write_sized_struct<T>(&mut self, address: usize, data: &T)
    where
        T: Serialize + Sized,
        [(); size_of::<T>()]: Sized,
    {
        let mut buffer = [0u8; size_of::<T>()];
        postcard::to_slice(data, &mut buffer).unwrap();
        self.write_buffer(address, &buffer);
    }

    pub fn read_sized_struct<T: DeserializeOwned>(&mut self, address: usize) -> T
    where
        [(); size_of::<T>()]: Sized,
    {
        let mut buffer = [0u8; size_of::<T>()];
        self.read_buffer(address, &mut buffer);
        postcard::from_bytes(&buffer).unwrap()
    }

    /// returns the size of the written data
    pub fn write_struct<T>(&mut self, address: usize, data: &T) -> usize
    where
        T: Serialize,
    {
        let mut buffer = [0u8; MAX_STRUCT_SIZE];
        match postcard::to_slice(data, &mut buffer) {
            Ok(len) => {
                crate::println!("Wrote {} bytes", len);
            }
            Err(err) => {
                crate::println!("Err: {:?}", err);
            }
        }
        self.write_buffer(address, &buffer);
        buffer.len() as usize
    }

    pub fn read_struct<T: DeserializeOwned>(&mut self, address: usize) -> T {
        let mut buffer = [0u8; MAX_STRUCT_SIZE];
        self.read_buffer(address, &mut buffer);
        postcard::from_bytes(&buffer).unwrap()
    }
}
