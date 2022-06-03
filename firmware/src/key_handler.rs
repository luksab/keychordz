use alloc::{string::String, vec::Vec};
use atmega32u4_usb_hid::{Finger, Key, Modifier, UsbKeyboard};
use serde::{Deserialize, Serialize};
use ufmt::derive::uDebug;

use crate::key_state::KeyState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UString(pub String);
impl ufmt::uWrite for UString {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), core::convert::Infallible> {
        self.0.push_str(s);
        Ok(())
    }
}
impl ufmt::uDisplay for UString {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        <str as ufmt::uDisplay>::fmt(&self.0, f)
    }
}
impl ufmt::uDebug for UString {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        <str as ufmt::uDisplay>::fmt(&self.0, f)
    }
}

#[derive(uDebug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum RGBAction {
    None,
    BrightnessSet(u8),
    BrightnessAdd(i8),
    Toggle,
}

#[derive(uDebug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Action {
    Key(Key),
    Word(UString),
    Layer(u8),
    RGBAction(RGBAction),
}

#[derive(uDebug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Chord {
    trigger: u16,
    key: Action,
}

impl Chord {
    /// Create a new chord.
    ///
    /// triggger are the Fingers bitwise ORed together.
    pub fn new(trigger: u16, key: Action) -> Self {
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

#[derive(uDebug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct ModifierKey {
    finger: Finger,
    modifier: Modifier,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layer {
    chords: Vec<Chord>,
    modifiers: Vec<ModifierKey>,
}

impl ufmt::uDebug for Layer {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.debug_list()?.entries(self.chords.iter())?.finish()?;

        f.debug_list()?.entries(self.modifiers.iter())?.finish()?;

        Ok(())
    }
}

impl Layer {
    pub fn default() -> Self {
        let chords = vec![
            Chord::new(Finger::LI as u16 | Finger::LM as u16, Action::Key(Key::R)),
            Chord::new(Finger::LI as u16 | Finger::LR as u16, Action::Key(Key::C)),
            Chord::new(Finger::LI as u16 | Finger::LP as u16, Action::Key(Key::F)),
            Chord::new(Finger::LI as u16 | Finger::RI as u16, Action::Key(Key::B)),
            Chord::new(Finger::LI as u16 | Finger::RM as u16, Action::Key(Key::V)),
            Chord::new(Finger::LI as u16 | Finger::RR as u16, Action::Key(Key::G)),
            Chord::new(
                Finger::LI as u16 | Finger::RP as u16,
                Action::Key(Key::Backspace),
            ),
            Chord::new(Finger::LM as u16 | Finger::LR as u16, Action::Key(Key::D)),
            Chord::new(Finger::LM as u16 | Finger::LP as u16, Action::Key(Key::X)),
            Chord::new(Finger::LM as u16 | Finger::RI as u16, Action::Key(Key::Y)),
            Chord::new(
                Finger::LM as u16 | Finger::RM as u16,
                Action::Key(Key::Comma),
            ),
            Chord::new(
                Finger::LM as u16 | Finger::RR as u16,
                Action::Key(Key::Minus),
            ),
            Chord::new(
                Finger::LM as u16 | Finger::RP as u16,
                Action::Key(Key::Quote),
            ),
            Chord::new(Finger::LR as u16 | Finger::LP as u16, Action::Key(Key::W)),
            Chord::new(Finger::LR as u16 | Finger::RI as u16, Action::Key(Key::J)),
            Chord::new(Finger::LR as u16 | Finger::RM as u16, Action::Key(Key::K)),
            Chord::new(
                Finger::LR as u16 | Finger::RR as u16,
                Action::Key(Key::Period),
            ),
            Chord::new(
                Finger::LR as u16 | Finger::RP as u16,
                Action::Key(Key::KpRightParen),
            ),
            Chord::new(Finger::LP as u16 | Finger::RI as u16, Action::Key(Key::Q)),
            Chord::new(Finger::LP as u16 | Finger::RM as u16, Action::Key(Key::Z)),
            Chord::new(
                Finger::LP as u16 | Finger::RR as u16,
                Action::Key(Key::KpLeftParen),
            ),
            Chord::new(Finger::RI as u16 | Finger::RM as u16, Action::Key(Key::H)),
            Chord::new(Finger::RI as u16 | Finger::RR as u16, Action::Key(Key::U)),
            Chord::new(Finger::RI as u16 | Finger::RP as u16, Action::Key(Key::M)),
            Chord::new(Finger::RM as u16 | Finger::RR as u16, Action::Key(Key::L)),
            Chord::new(
                Finger::RR as u16 | Finger::RP as u16,
                Action::Key(Key::Semicolon),
            ),
            Chord::new(Finger::LP as u16, Action::Key(Key::A)),
            Chord::new(Finger::LR as u16, Action::Key(Key::S)),
            Chord::new(Finger::LM as u16, Action::Key(Key::E)),
            Chord::new(Finger::LI as u16, Action::Key(Key::T)),
            Chord::new(Finger::LU as u16, Action::Key(Key::Tab)),
            Chord::new(Finger::LL as u16, Action::Key(Key::Space)),
            Chord::new(Finger::RI as u16, Action::Key(Key::N)),
            Chord::new(Finger::RM as u16, Action::Key(Key::I)),
            Chord::new(Finger::RR as u16, Action::Key(Key::O)),
            Chord::new(Finger::RP as u16, Action::Key(Key::P)),
            Chord::new(Finger::RU as u16, Action::Key(Key::Esc)),
            Chord::new(Finger::RL as u16, Action::Key(Key::Space)),
        ];

        Layer {
            chords,
            modifiers: vec![
                ModifierKey {
                    finger: Finger::LD,
                    modifier: Modifier::Shift,
                },
                ModifierKey {
                    finger: Finger::RD,
                    modifier: Modifier::Shift,
                },
            ],
        }
    }

    pub fn empty() -> Self {
        Layer {
            chords: vec![],
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
    pub fn new(layers: Vec<Layer>) -> Self {
        KeyHandler {
            layers,
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
                    match chord.key {
                        Action::Key(key) => {
                            let _ = UsbKeyboard::press_key(key, modifier);
                        }
                        _ => {}
                    }
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
