
/// simple wrapper around a few bitwise operations
pub struct KeyState {
    last_state: u16,
    state: u16,
    just_pressed: u16,
}

impl KeyState {
    pub fn new() -> Self {
        Self {
            last_state: 0,
            state: 0,
            just_pressed: 0,
        }
    }

    /// update state with bitfields of the keys pressed per side
    /// 
    /// returns the keys that were just pressed
    pub fn update(&mut self, left: u8, right: u8) -> u16 {
        self.last_state = self.state;
        self.state = (left as u16) << 8 | (right as u16);
        self.just_pressed = self.state & !self.last_state;
        self.just_pressed
    }

    /// get the current state
    pub fn get_state(&self) -> u16 {
        self.state
    }

    /// get the last state
    pub fn get_last_state(&self) -> u16 {
        self.last_state
    }

    /// get which keys have been pressed since the last update
    pub fn get_pressed(&self) -> u16 {
        self.just_pressed
    }
}
