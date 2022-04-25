use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{collections::HashSet, fmt, str::FromStr};
use strum::{EnumCount, VariantNames};
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumString, EnumVariantNames};

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
    EnumCountMacro,
)]
#[repr(u8)]
enum Finger {
    LP,
    LR,
    LM,
    LI,
    RI,
    RM,
    RR,
    RP,
    LU,
    LD,
    LL,
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
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let mut words = line.split_whitespace();
            let mut fingers = Vec::new();
            let mut key = None;
            while let Some(finger) = words.next() {
                let finger = match Finger::from_str(finger.to_uppercase().as_str()) {
                    Ok(f) => f,
                    Err(_) => {
                        // return Err(format!(
                        //     "Invalid finger name at line {}: {}",
                        //     line_num, finger
                        // ))
                        key = Some(finger.to_uppercase());
                        break;
                    }
                };
                fingers.push(finger);
            }
            if fingers.len() == 0 {
                return Err(format!("Invalid finger on line {}", line_num));
            }
            let key = key.ok_or(format!("Missing Key: line {}", line_num))?;
            // if key.len() != 1 {
            //     return Err(format!(
            //         "Key on line {} must be a single character: {}",
            //         line_num, key
            //     ));
            // }
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

    fn check_dup_in_combo(&self) -> Result<(), String> {
        for combo in &self.combos {
            let mut finger_set = HashSet::new();
            for finger in &combo.fingers {
                if !finger_set.insert(finger.clone()) {
                    return Err(format!("Duplicate finger in combo: {}", combo));
                }
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
        self.check_dup_in_combo()?;
        self.check_all_single()?;
        Ok(())
    }
}

impl Config {
    fn get_option_finger_lookup(&self) -> Result<Vec<Option<String>>, String> {
        let mut single_key_lookup = Finger::VARIANTS.iter().map(|_| None).collect::<Vec<_>>();

        for combo in &self.combos {
            if combo.fingers.len() != 1 {
                continue;
            }
            let finger = combo.fingers[0];
            match &single_key_lookup[finger as usize] {
                Some(other_key) => {
                    return Err(format!(
                        "Finger {} is already mapped to {}",
                        finger, other_key
                    ));
                }
                None => {
                    single_key_lookup[finger as usize] = Some(combo.key.clone());
                }
            }
        }

        // single_key_lookup.iter().map(|x| x.unwrap_or(' ')).collect()
        Ok(single_key_lookup)
    }

    fn get_finger_lookup(&self) -> Result<Vec<String>, String> {
        let mut finger_lookup = self.get_option_finger_lookup()?;
        for (i, key) in finger_lookup.iter_mut().enumerate() {
            if key.is_none() {
                return Err(format!(
                    "Finger {} is not mapped",
                    Finger::try_from(i as u8).unwrap()
                ));
            }
        }
        Ok(finger_lookup.into_iter().map(|x| x.unwrap()).collect())
    }

    fn to_keymap(&self) -> Result<String, String> {
        let mut out = String::new();
        out += "[0] = LAYOUT_keychordz(\n";

        let single_key_lookup = self.get_finger_lookup()?;

        out += "   ";
        for (i, key) in single_key_lookup.iter().enumerate() {
            if i == 3 {
                out += &format!("    {},                  ", key);
            } else if i == 7 {
                out += &format!("    {}, \\\n                            ", key);
            } else if i == 10 {
                out += &format!(" {},           ", key);
            } else if i == Finger::COUNT - 1 {
                out += &format!(" {}", key);
            } else if i > 7 {
                out += &format!(" {},", key);
            } else {
                out += &format!("    {},", key);
            }
        }
        out += " \\\n       )";

        Ok(out)
    }

    /// const uint16_t PROGMEM test_combo1[] = {A, S, COMBO_END};
    /// const uint16_t PROGMEM test_combo2[] = {F, H, COMBO_END};
    /// combo_t key_combos[COMBO_COUNT] = {
    ///     COMBO(test_combo1, ESC),
    ///     COMBO(test_combo2, LCTL(Y)), // keycodes with modifiers are possible too!
    /// };
    fn to_qmk_combos(&self) -> Result<String, String> {
        let single_key_lookup = self.get_finger_lookup()?;
        let mut progmem_out = String::new();

        for (i, combo) in self.combos.iter().enumerate() {
            if combo.fingers.len() < 2 {
                continue;
            }
            let mut out = String::new();
            out += &format!("const uint16_t PROGMEM combo_{}[] = {{", i);
            for finger in &combo.fingers {
                out += &format!(" {},", single_key_lookup[*finger as usize]);
            }
            out += " COMBO_END};\n";
            progmem_out += &out;
        }

        let mut key_combos_out = String::new();
        key_combos_out += "combo_t key_combos[COMBO_COUNT] = {\n";
        for (i, combo) in self.combos.iter().enumerate() {
            if combo.fingers.len() < 2 {
                continue;
            }
            let mut out = String::new();
            out += &format!("    COMBO(combo_{}, ", i);
            out += &format!("{}),\n", combo.key);

            key_combos_out += &out;
        }
        key_combos_out += "};\n";

        Ok(format!(
            "{}\n\n{}\n\nComboCount = {}",
            progmem_out,
            key_combos_out,
            self.combos.len() - Finger::COUNT,
        ))
    }
}

fn main() {
    // read filename from first argument
    let file_name = std::env::args().nth(1).unwrap();
    let config = Config::from_str(&std::fs::read_to_string(file_name).unwrap()).unwrap();
    println!("{}", config);
    match config.check() {
        Ok(_) => println!("Config is valid"),
        Err(e) => {
            println!("Config is invalid: {}", e);
            std::process::exit(1);
        }
    }
    println!("{}", config.to_keymap().unwrap());
    println!("{}", config.to_qmk_combos().unwrap());
}
