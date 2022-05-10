use alloc::vec::Vec;
use atmega32u4_usb_hid::{Key, Modifier, UsbKeyboard};

use crate::key_state::KeyState;

struct Chord {
    trigger: u16,
    key: Key,
}

impl Chord {
    pub fn new(trigger: u16, key: Key) -> Self {
        Chord { trigger, key }
    }

    /// Trigger if just_released in chord and chord in last_state
    ///
    /// For one number to be "in" another number, the locical and of the two has to eqal the first number
    pub fn triggers(&self, last_state: u16, just_released: u16) -> bool {
        just_released != 0
            && just_released & self.trigger == just_released
            && self.trigger & last_state == self.trigger
    }
}

struct Layer {
    chords: Vec<Chord>,
}

impl Layer {
    pub fn default() -> Self {
        Layer {
            chords: vec![
                Chord::new(0b0100000000000001, Key::A),
                Chord::new(0b0100000000000000, Key::B),
            ],
        }
    }
}

/// chords trigger on release of any key in the chord
pub struct KeyHandler {
    layers: Vec<Layer>,
    active_layer: usize,
    pub state: KeyState,
    /// false after a key is released, so that shorter chords are not triggered
    /// when releasing a key from a longer chord
    pub should_trigger: bool,
}

impl KeyHandler {
    pub fn new() -> Self {
        KeyHandler {
            layers: vec![Layer::default()],
            active_layer: 0,
            state: KeyState::new(),
            should_trigger: false,
        }
    }

    pub fn update(&mut self, left: u8, right: u8) {
        self.state.update(left, right);

        if self.should_trigger {
            for chord in self.layers[self.active_layer].chords.iter() {
                if chord.triggers(self.state.last_state, self.state.just_released) {
                    // TODO: handle error
                    let _ = UsbKeyboard::press_key(chord.key, Modifier::None);
                    // only trigger for the longest chord
                    // chors are sorted by trigger length
                    break;
                }
            }
        }

        // If a key was pressed, we should trigger chords
        self.should_trigger |= self.state.just_pressed != 0;

        // don't trigger chords if a key is released
        if self.state.just_released != 0 {
            self.should_trigger = false;
        }
    }
}
