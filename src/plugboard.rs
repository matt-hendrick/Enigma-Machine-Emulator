use crate::charindex::char_to_index;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Plugboard {
    wiring: HashMap<u8, u8>,
}

impl Plugboard {
    pub fn new(letter_mapping: &str) -> Self {
        Plugboard {
            wiring: generate_wiring_hashmap(letter_mapping),
        }
    }

    pub fn encode(&self, index: u8) -> u8 {
        if self.wiring.contains_key(&index) {
            *self.wiring.get(&index).unwrap()
        } else {
            index
        }
    }
}

fn generate_wiring_hashmap(letter_mapping: &str) -> HashMap<u8, u8> {
    let mut hashmap: HashMap<u8, u8> = HashMap::new();

    if letter_mapping.len() < 2 {
        return hashmap;
    }
    let string_bytes: &[u8] = letter_mapping.as_bytes();

    let mut index: usize = 0;

    while index < string_bytes.len() - 1 && hashmap.len() < 20 {
        if !(string_bytes[index] as char).is_ascii_uppercase() {
            index += 1;
        } else {
            // maps first letter to second
            hashmap.insert(
                char_to_index(string_bytes[index] as char),
                char_to_index(string_bytes[index + 1] as char),
            );
            // maps second letter to first
            hashmap.insert(
                char_to_index(string_bytes[index + 1] as char),
                char_to_index(string_bytes[index] as char),
            );
            index += 2;
        }
    }
    return hashmap;
}
