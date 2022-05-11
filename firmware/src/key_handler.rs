use alloc::vec::Vec;
use atmega32u4_usb_hid::{Finger, Key, Modifier, UsbKeyboard};

use crate::key_state::KeyState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// implement ord based on the number of bits set in the chord
impl Ord for Chord {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.trigger.count_ones().cmp(&other.trigger.count_ones())
    }
}

impl PartialOrd for Chord {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModifierKey {
    finger: Finger,
    modifier: Modifier,
}

struct Layer {
    chords: Vec<Chord>,
    modifiers: Vec<ModifierKey>,
}

impl Layer {
    pub fn default() -> Self {
        let mut chords = vec![
            Chord::new(0b0100000000000000, Key::A),
            Chord::new(0b0010000000000000, Key::S),
            Chord::new(0b0001000000000000, Key::E),
            Chord::new(0b0000100000000000, Key::T),
            Chord::new(0b0000010000000000, Key::Tab),
            Chord::new(0b0000000100000000, Key::Space),
            Chord::new(0b0000000000001000, Key::N),
            Chord::new(0b0000000000010000, Key::I),
            Chord::new(0b0000000000100000, Key::O),
            Chord::new(0b0000000001000000, Key::P),
            Chord::new(0b0000000000000100, Key::Esc),
            Chord::new(0b0000000000000001, Key::Space),
            Chord::new(0b0001100000000000, Key::R),
            Chord::new(0b0010100000000000, Key::C),
            Chord::new(0b0100100000000000, Key::F),
            Chord::new(0b0000100000001000, Key::B),
            Chord::new(0b0000100000010000, Key::V),
            Chord::new(0b0000100000100000, Key::G),
            Chord::new(0b0000100001000000, Key::Backspace),
            Chord::new(0b0011000000000000, Key::D),
            Chord::new(0b0101000000000000, Key::X),
            Chord::new(0b0001000000001000, Key::Y),
            Chord::new(0b0001000000010000, Key::Comma),
            Chord::new(0b0001000000100000, Key::Minus),
            Chord::new(0b0001000001000000, Key::Quote),
            Chord::new(0b0110000000000000, Key::W),
            Chord::new(0b0010000000001000, Key::J),
            Chord::new(0b0010000000010000, Key::K),
            Chord::new(0b0010000000100000, Key::Period),
            Chord::new(0b0010000001000000, Key::KpRightParen),
            Chord::new(0b0100000000001000, Key::Q),
            Chord::new(0b0100000000010000, Key::Z),
            Chord::new(0b0100000000100000, Key::KpLeftParen),
            Chord::new(0b0000000000011000, Key::H),
            Chord::new(0b0000000000101000, Key::U),
            Chord::new(0b0000000001001000, Key::M),
            Chord::new(0b0000000000110000, Key::L),
            Chord::new(0b0000000001100000, Key::Semicolon),
        ];
        // reverse sort chords
        chords.sort_by(|a, b| b.cmp(a));

        Layer {
            chords,
            modifiers: vec![],
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

        let mut modifier = Modifier::None as u8;

        for finger in &self.layers[self.active_layer].modifiers {
            if self.state.state & finger.finger as u16 != 0 {
                modifier |= finger.modifier as u8;
            }
        }

        if self.should_trigger {
            for chord in self.layers[self.active_layer].chords.iter() {
                if chord.triggers(self.state.last_state, self.state.just_released) {
                    // TODO: handle error
                    let _ = UsbKeyboard::press_key(chord.key, modifier);
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
