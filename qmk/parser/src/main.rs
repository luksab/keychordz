use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{collections::HashSet, fmt, str::FromStr};
use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};

// [L/R] [Pinkie, Ring, Middle, Index, thumbL, thumbU, thumbD]
#[derive(
    EnumString,
    Display,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    TryFromPrimitive,
    EnumVariantNames,
)]
#[repr(u8)]
enum Finger {
    LP,
    LR,
    LM,
    LI,
    LU,
    LD,
    LL,
    RP,
    RR,
    RM,
    RI,
    RU,
    RD,
    RL,
}

impl fmt::Debug for Finger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Combo {
    fingers: Vec<Finger>,
    key: String,
}

impl fmt::Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, finger) in self.fingers.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", finger)?;
            } else {
                write!(f, " {}", finger)?;
            }
        }
        write!(f, " {}", self.key)
    }
}

#[derive(Debug)]
struct Config {
    combos: Vec<Combo>,
}

impl FromStr for Config {
    type Err = String;
    /// format: `([L/R][Pinkie, Ring, Middle, Index, thumbL, thumbU, thumbD]+ [key]\n)+`
    fn from_str(config: &str) -> Result<Self, Self::Err> {
        let mut combos = Vec::new();
        let mut lines = config.lines();
        let mut line_num = 0;
        while let Some(line) = lines.next() {
            line_num += 1;
            if line.starts_with('#') {
                continue;
            }
            let mut words = line.split_whitespace();
            let mut fingers = Vec::new();
            let mut key = None;
            while let Some(finger) = words.next() {
                let finger = match Finger::from_str(finger) {
                    Ok(f) => f,
                    Err(_) => {
                        // return Err(format!(
                        //     "Invalid finger name at line {}: {}",
                        //     line_num, finger
                        // ))
                        key = Some(finger.to_string());
                        break;
                    }
                };
                fingers.push(finger);
            }
            if fingers.len() == 0 {
                return Err(format!("Invalid finger on line {}", line_num));
            }
            let key = key.ok_or(format!("Missing Key: line {}", line_num))?;
            if key.len() != 1 {
                return Err(format!(
                    "Key on line {} must be a single character: {}",
                    line_num, key
                ));
            }
            combos.push(Combo {
                fingers,
                key: key.to_string(),
            });
        }
        Ok(Config { combos })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // don't put a newline at the last line
        for (i, combo) in self.combos.iter().enumerate() {
            if i == self.combos.len() - 1 {
                write!(f, "{}", combo)?;
            } else {
                write!(f, "{}\n", combo)?;
            }
        }
        Ok(())
    }
}

impl Config {
    fn check_dup(&self) -> Result<(), String> {
        let mut finger_set = HashSet::new();
        for combo in &self.combos {
            if !finger_set.insert(combo.fingers.clone()) {
                return Err(format!("Duplicate combo: {}", combo));
            }
        }
        Ok(())
    }

    fn check_all_single(&self) -> Result<(), String> {
        match self.get_finger_lookup() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn check(&self) -> Result<(), String> {
        self.check_dup()?;
        self.check_all_single()?;
        Ok(())
    }
}

impl Config {
    fn get_option_finger_lookup(&self) -> Result<Vec<Option<char>>, String> {
        let mut single_key_lookup = Finger::VARIANTS.iter().map(|_| None).collect::<Vec<_>>();

        for combo in &self.combos {
            if combo.fingers.len() != 1 {
                continue;
            }
            let finger = combo.fingers[0];
            let key = combo.key.chars().next().unwrap();
            match single_key_lookup[finger as usize] {
                Some(other_key) => {
                    return Err(format!(
                        "Finger {} is already mapped to {}",
                        finger, other_key
                    ));
                }
                None => {
                    single_key_lookup[finger as usize] = Some(key);
                }
            }
        }

        // single_key_lookup.iter().map(|x| x.unwrap_or(' ')).collect()
        Ok(single_key_lookup)
    }

    fn get_finger_lookup(&self) -> Result<Vec<char>, String> {
        let mut finger_lookup = self.get_option_finger_lookup()?;
        for (i, key) in finger_lookup.iter_mut().enumerate() {
            if key.is_none() {
                return Err(format!(
                    "Finger {} is not mapped",
                    Finger::try_from(i as u8).unwrap()
                ));
            }
        }
        Ok(finger_lookup.iter().map(|x| x.unwrap()).collect())
    }

    #[allow(non_snake_case)]
    fn to_keymap(self) -> String {
        let mut out = String::new();
        out += "[0] = LAYOUT_keychordz(\n";

        let single_key_lookup = self.get_finger_lookup();

        println!("{:?}", single_key_lookup);

        out
    }
}

fn main() {
    let config = Config::from_str(include_str!("../../config.cfg")).unwrap();
    println!("{}", config);
    match config.check() {
        Ok(_) => println!("Config is valid"),
        Err(e) => println!("Config is invalid: {}", e),
    }
    println!("{}", config.to_keymap());
}
