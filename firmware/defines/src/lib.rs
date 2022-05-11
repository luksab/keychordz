#![no_std]

#[macro_use]
extern crate alloc;

use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Finger {
    LP = 0b0100000000000000,
    LR = 0b0010000000000000,
    LM = 0b0001000000000000,
    LI = 0b0000100000000000,
    LU = 0b0000010000000000,
    LD = 0b0000001000000000,
    LL = 0b0000000100000000,

    RP = 0b0000000001000000,
    RR = 0b0000000000100000,
    RM = 0b0000000000010000,
    RI = 0b0000000000001000,
    RU = 0b0000000000000100,
    RD = 0b0000000000000010,
    RL = 0b0000000000000001,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Modifier {
    None = 0,
    Ctrl = 0x01,
    Shift = 0x02,
    Alt = 0x04,
    Gui = 0x08,
    RightCtrl = 0x10,
    RightShift = 0x20,
    RightAlt = 0x40,
    RightGui = 0x80,
}

impl FromStr for Modifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "ctrl" => Ok(Modifier::Ctrl),
            "shift" => Ok(Modifier::Shift),
            "alt" => Ok(Modifier::Alt),
            "gui" => Ok(Modifier::Gui),
            "rightctrl" => Ok(Modifier::RightCtrl),
            "rightshift" => Ok(Modifier::RightShift),
            "rightalt" => Ok(Modifier::RightAlt),
            "rightgui" => Ok(Modifier::RightGui),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Key {
    None = 0,
    ErrOvf = 0x01,
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    Num1 = 30,
    Num2 = 31,
    Num3 = 32,
    Num4 = 33,
    Num5 = 34,
    Num6 = 35,
    Num7 = 36,
    Num8 = 37,
    Num9 = 38,
    Num0 = 39,
    Enter = 40,
    Esc = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equal = 46,
    LeftBrace = 47,
    RightBrace = 48,
    Backslash = 49,
    Number = 50,
    Semicolon = 51,
    Quote = 52,
    Tilde = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    Printscreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
    NumLock = 83,
    PadSlash = 84,
    PadAsterix = 85,
    PadMinus = 86,
    PadPlus = 87,
    PadEnter = 88,
    Keypad1 = 89,
    Keypad2 = 90,
    Keypad3 = 91,
    Keypad4 = 92,
    Keypad5 = 93,
    Keypad6 = 94,
    Keypad7 = 95,
    Keypad8 = 96,
    Keypad9 = 97,
    Keypad0 = 98,
    KeypadPeriod = 99,

    Key102nd = 0x64, // Keyboard Non-US \ and |
    Compose = 0x65,  // Keyboard Application
    Power = 0x66,    // Keyboard Power
    Kpequal = 0x67,  // Keypad =

    F13 = 0x68, // Keyboard F13
    F14 = 0x69, // Keyboard F14
    F15 = 0x6a, // Keyboard F15
    F16 = 0x6b, // Keyboard F16
    F17 = 0x6c, // Keyboard F17
    F18 = 0x6d, // Keyboard F18
    F19 = 0x6e, // Keyboard F19
    F20 = 0x6f, // Keyboard F20
    F21 = 0x70, // Keyboard F21
    F22 = 0x71, // Keyboard F22
    F23 = 0x72, // Keyboard F23
    F24 = 0x73, // Keyboard F24

    Open = 0x74,       // Keyboard Execute
    Help = 0x75,       // Keyboard Help
    Props = 0x76,      // Keyboard Menu
    Front = 0x77,      // Keyboard Select
    Stop = 0x78,       // Keyboard Stop
    Again = 0x79,      // Keyboard Again
    Undo = 0x7a,       // Keyboard Undo
    Cut = 0x7b,        // Keyboard Cut
    Copy = 0x7c,       // Keyboard Copy
    Paste = 0x7d,      // Keyboard Paste
    Find = 0x7e,       // Keyboard Find
    Mute = 0x7f,       // Keyboard Mute
    Volumeup = 0x80,   // Keyboard Volume Up
    Volumedown = 0x81, // Keyboard Volume Down
    // 0x82  Keyboard Locking Caps Lock
    // 0x83  Keyboard Locking Num Lock
    // 0x84  Keyboard Locking Scroll Lock
    KeypadComma = 0x85, // Keypad Comma
    // 0x86  Keypad Equal Sign
    Ro = 0x87,               // Keyboard International1
    Katakanahiragana = 0x88, // Keyboard International2
    Yen = 0x89,              // Keyboard International3
    Henkan = 0x8a,           // Keyboard International4
    Muhenkan = 0x8b,         // Keyboard International5
    KpJpComma = 0x8c,        // Keyboard International6
    // 0x8d  Keyboard International7
    // 0x8e  Keyboard International8
    // 0x8f  Keyboard International9
    Hangeul = 0x90,        // Keyboard LANG1
    Hanja = 0x91,          // Keyboard LANG2
    Katakana = 0x92,       // Keyboard LANG3
    Hiragana = 0x93,       // Keyboard LANG4
    Zenkakuhankaku = 0x94, // Keyboard LANG5
    // 0x95  Keyboard LANG6
    // 0x96  Keyboard LANG7
    // 0x97  Keyboard LANG8
    // 0x98  Keyboard LANG9
    // 0x99  Keyboard Alternate Erase
    // 0x9a  Keyboard SysReq/Attention
    // 0x9b  Keyboard Cancel
    // 0x9c  Keyboard Clear
    // 0x9d  Keyboard Prior
    // 0x9e  Keyboard Return
    // 0x9f  Keyboard Separator
    // 0xa0  Keyboard Out
    // 0xa1  Keyboard Oper
    // 0xa2  Keyboard Clear/Again
    // 0xa3  Keyboard CrSel/Props
    // 0xa4  Keyboard ExSel

    // 0xb0  Keypad 00
    // 0xb1  Keypad 000
    // 0xb2  Thousands Separator
    // 0xb3  Decimal Separator
    // 0xb4  Currency Unit
    // 0xb5  Currency Sub-unit
    KpLeftParen = 0xb6,  // Keypad (
    KpRightParen = 0xb7, // Keypad )
    // 0xb8  Keypad {
    // 0xb9  Keypad }
    // 0xba  Keypad Tab
    // 0xbb  Keypad Backspace
    // 0xbc  Keypad A
    // 0xbd  Keypad B
    // 0xbe  Keypad C
    // 0xbf  Keypad D
    // 0xc0  Keypad E
    // 0xc1  Keypad F
    // 0xc2  Keypad XOR
    // 0xc3  Keypad ^
    // 0xc4  Keypad %
    // 0xc5  Keypad <
    // 0xc6  Keypad >
    // 0xc7  Keypad &
    // 0xc8  Keypad &&
    // 0xc9  Keypad |
    // 0xca  Keypad ||
    // 0xcb  Keypad :
    // 0xcc  Keypad #
    // 0xcd  Keypad Space
    // 0xce  Keypad @
    // 0xcf  Keypad !
    // 0xd0  Keypad Memory Store
    // 0xd1  Keypad Memory Recall
    // 0xd2  Keypad Memory Clear
    // 0xd3  Keypad Memory Add
    // 0xd4  Keypad Memory Subtract
    // 0xd5  Keypad Memory Multiply
    // 0xd6  Keypad Memory Divide
    // 0xd7  Keypad +/-
    // 0xd8  Keypad Clear
    // 0xd9  Keypad Clear Entry
    // 0xda  Keypad Binary
    // 0xdb  Keypad Octal
    // 0xdc  Keypad Decimal
    // 0xdd  Keypad Hexadecimal
    LeftCtrl = 0xe0,   // Keyboard Left Control
    LeftShift = 0xe1,  // Keyboard Left Shift
    LeftAlt = 0xe2,    // Keyboard Left Alt
    LeftMeta = 0xe3,   // Keyboard Left GUI
    RightCtrl = 0xe4,  // Keyboard Right Control
    RightShift = 0xe5, // Keyboard Right Shift
    RightAlt = 0xe6,   // Keyboard Right Alt
    RightMeta = 0xe7,  // Keyboard Right GUI
}

impl FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_ref() {
            "a" => Ok(Key::A),
            "b" => Ok(Key::B),
            "c" => Ok(Key::C),
            "d" => Ok(Key::D),
            "e" => Ok(Key::E),
            "f" => Ok(Key::F),
            "g" => Ok(Key::G),
            "h" => Ok(Key::H),
            "i" => Ok(Key::I),
            "j" => Ok(Key::J),
            "k" => Ok(Key::K),
            "l" => Ok(Key::L),
            "m" => Ok(Key::M),
            "n" => Ok(Key::N),
            "o" => Ok(Key::O),
            "p" => Ok(Key::P),
            "q" => Ok(Key::Q),
            "r" => Ok(Key::R),
            "s" => Ok(Key::S),
            "t" => Ok(Key::T),
            "u" => Ok(Key::U),
            "v" => Ok(Key::V),
            "w" => Ok(Key::W),
            "x" => Ok(Key::X),
            "y" => Ok(Key::Y),
            "z" => Ok(Key::Z),
            "0" => Ok(Key::Num0),
            "num0" => Ok(Key::Num0),
            "1" => Ok(Key::Num1),
            "num1" => Ok(Key::Num1),
            "2" => Ok(Key::Num2),
            "num2" => Ok(Key::Num2),
            "3" => Ok(Key::Num3),
            "num3" => Ok(Key::Num3),
            "4" => Ok(Key::Num4),
            "num4" => Ok(Key::Num4),
            "5" => Ok(Key::Num5),
            "num5" => Ok(Key::Num5),
            "6" => Ok(Key::Num6),
            "num6" => Ok(Key::Num6),
            "7" => Ok(Key::Num7),
            "num7" => Ok(Key::Num7),
            "8" => Ok(Key::Num8),
            "num8" => Ok(Key::Num8),
            "9" => Ok(Key::Num9),
            "num9" => Ok(Key::Num9),
            "f1" => Ok(Key::F1),
            "f2" => Ok(Key::F2),
            "f3" => Ok(Key::F3),
            "f4" => Ok(Key::F4),
            "f5" => Ok(Key::F5),
            "f6" => Ok(Key::F6),
            "f7" => Ok(Key::F7),
            "f8" => Ok(Key::F8),
            "f9" => Ok(Key::F9),
            "f10" => Ok(Key::F10),
            "f11" => Ok(Key::F11),
            "f12" => Ok(Key::F12),
            "f13" => Ok(Key::F13),
            "f14" => Ok(Key::F14),
            "f15" => Ok(Key::F15),
            "f16" => Ok(Key::F16),
            "f17" => Ok(Key::F17),
            "f18" => Ok(Key::F18),
            "f19" => Ok(Key::F19),
            "f20" => Ok(Key::F20),
            "f21" => Ok(Key::F21),
            "f22" => Ok(Key::F22),
            "f23" => Ok(Key::F23),
            "f24" => Ok(Key::F24),
            "enter" => Ok(Key::Enter),
            "esc" => Ok(Key::Esc),
            "backspace" => Ok(Key::Backspace),
            "tab" => Ok(Key::Tab),
            "space" => Ok(Key::Space),
            "minus" => Ok(Key::Minus),
            "equal" => Ok(Key::Equal),
            "leftbrace" => Ok(Key::LeftBrace),
            "rightBrace" => Ok(Key::RightBrace),
            "backslash" => Ok(Key::Backslash),
            "number" => Ok(Key::Number),
            "semicolon" => Ok(Key::Semicolon),
            "quote" => Ok(Key::Quote),
            "tilde" => Ok(Key::Tilde),
            "comma" => Ok(Key::Comma),
            "period" => Ok(Key::Period),
            "dot" => Ok(Key::Period),
            "slash" => Ok(Key::Slash),
            "capslock" => Ok(Key::CapsLock),
            "open" => Ok(Key::Open),
            "help" => Ok(Key::Help),
            "props" => Ok(Key::Props),
            "front" => Ok(Key::Front),
            "stop" => Ok(Key::Stop),
            "again" => Ok(Key::Again),
            "undo" => Ok(Key::Undo),
            "cut" => Ok(Key::Cut),
            "copy" => Ok(Key::Copy),
            "paste" => Ok(Key::Paste),
            "find" => Ok(Key::Find),
            "mute" => Ok(Key::Mute),
            "volumeup" => Ok(Key::Volumeup),
            "volumedown" => Ok(Key::Volumedown),
            "kpcomma" => Ok(Key::KeypadComma),
            "keypadcomma" => Ok(Key::KeypadComma),
            "ro" => Ok(Key::Ro),
            "katakanahiragana" => Ok(Key::Katakanahiragana),
            "yen" => Ok(Key::Yen),
            "henkan" => Ok(Key::Henkan),
            "muhenkan" => Ok(Key::Muhenkan),
            "kpfpcomma" => Ok(Key::KpJpComma),
            "hangeul" => Ok(Key::Hangeul),
            "hanja" => Ok(Key::Hanja),
            "katakana" => Ok(Key::Katakana),
            "hiragana" => Ok(Key::Hiragana),
            "zenkakuhankaku" => Ok(Key::Zenkakuhankaku),
            "kpleftparen" => Ok(Key::KpLeftParen),
            "kprightparen" => Ok(Key::KpRightParen),
            // TODO: make normal paren from these
            "leftparen" => Ok(Key::KpLeftParen),
            "rightparen" => Ok(Key::KpRightParen),
            "left_paren" => Ok(Key::KpLeftParen),
            "right_paren" => Ok(Key::KpRightParen),

            "leftctrl" => Ok(Key::LeftCtrl),
            "leftshift" => Ok(Key::LeftShift),
            "leftalt" => Ok(Key::LeftAlt),
            "leftmeta" => Ok(Key::LeftMeta),
            "rightctrl" => Ok(Key::RightCtrl),
            "rightshift" => Ok(Key::RightShift),
            "rightalt" => Ok(Key::RightAlt),
            "rightmeta" => Ok(Key::RightMeta),
            _ => Err(()),
        }
    }
}
