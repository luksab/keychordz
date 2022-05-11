use alloc::vec::Vec;
use atmega32u4_usb_hid::{Finger, Key, Modifier, UsbKeyboard};

use crate::key_state::KeyState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chord {
    trigger: u16,
    key: Key,
}

impl Chord {
    /// Create a new chord.
    ///
    /// triggger are the Fingers bitwise ORed together.
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
        let chords = vec![
            Chord::new(Finger::LI as u16 | Finger::LM as u16, Key::R),
            Chord::new(Finger::LI as u16 | Finger::LR as u16, Key::C),
            Chord::new(Finger::LI as u16 | Finger::LP as u16, Key::F),
            Chord::new(Finger::LI as u16 | Finger::RI as u16, Key::B),
            Chord::new(Finger::LI as u16 | Finger::RM as u16, Key::V),
            Chord::new(Finger::LI as u16 | Finger::RR as u16, Key::G),
            Chord::new(Finger::LI as u16 | Finger::RP as u16, Key::Backspace),
            Chord::new(Finger::LM as u16 | Finger::LR as u16, Key::D),
            Chord::new(Finger::LM as u16 | Finger::LP as u16, Key::X),
            Chord::new(Finger::LM as u16 | Finger::RI as u16, Key::Y),
            Chord::new(Finger::LM as u16 | Finger::RM as u16, Key::Comma),
            Chord::new(Finger::LM as u16 | Finger::RR as u16, Key::Minus),
            Chord::new(Finger::LM as u16 | Finger::RP as u16, Key::Quote),
            Chord::new(Finger::LR as u16 | Finger::LP as u16, Key::W),
            Chord::new(Finger::LR as u16 | Finger::RI as u16, Key::J),
            Chord::new(Finger::LR as u16 | Finger::RM as u16, Key::K),
            Chord::new(Finger::LR as u16 | Finger::RR as u16, Key::Period),
            Chord::new(Finger::LR as u16 | Finger::RP as u16, Key::KpRightParen),
            Chord::new(Finger::LP as u16 | Finger::RI as u16, Key::Q),
            Chord::new(Finger::LP as u16 | Finger::RM as u16, Key::Z),
            Chord::new(Finger::LP as u16 | Finger::RR as u16, Key::KpLeftParen),
            Chord::new(Finger::RI as u16 | Finger::RM as u16, Key::H),
            Chord::new(Finger::RI as u16 | Finger::RR as u16, Key::U),
            Chord::new(Finger::RI as u16 | Finger::RP as u16, Key::M),
            Chord::new(Finger::RM as u16 | Finger::RR as u16, Key::L),
            Chord::new(Finger::RR as u16 | Finger::RP as u16, Key::Semicolon),
            Chord::new(Finger::LP as u16, Key::A),
            Chord::new(Finger::LR as u16, Key::S),
            Chord::new(Finger::LM as u16, Key::E),
            Chord::new(Finger::LI as u16, Key::T),
            Chord::new(Finger::LU as u16, Key::Tab),
            Chord::new(Finger::LL as u16, Key::Space),
            Chord::new(Finger::RI as u16, Key::N),
            Chord::new(Finger::RM as u16, Key::I),
            Chord::new(Finger::RR as u16, Key::O),
            Chord::new(Finger::RP as u16, Key::P),
            Chord::new(Finger::RU as u16, Key::Esc),
            Chord::new(Finger::RL as u16, Key::Space),
        ];

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
