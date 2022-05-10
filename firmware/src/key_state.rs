use ufmt::derive::uDebug;

/// simple wrapper around a few bitwise operations
#[derive(uDebug)]
pub struct KeyState {
    pub last_state: u16,
    pub state: u16,
    pub just_pressed: u16,
    pub just_released: u16,
}

impl KeyState {
    pub fn new() -> Self {
        Self {
            last_state: 0,
            state: 0,
            just_pressed: 0,
            just_released: 0,
        }
    }

    /// Update state with bitfields of the keys pressed per side
    /// 
    /// returns the just pressed keys
    pub fn update(&mut self, left: u8, right: u8) -> u16 {
        self.last_state = self.state;
        self.state = (left as u16) << 8 | (right as u16);
        self.just_pressed = self.state & !self.last_state;
        self.just_released = !self.state & self.last_state;
        self.just_pressed
    }
}
