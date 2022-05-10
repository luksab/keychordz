use arduino_hal::pac::EEPROM;
struct EEPROMWriter {
    eeprom_registers: EEPROM,
}

impl EEPROMWriter {
    pub fn new(eeprom_registers: EEPROM) -> EEPROMWriter {
        EEPROMWriter { eeprom_registers }
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
}
