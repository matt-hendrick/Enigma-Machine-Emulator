use std::collections::HashMap;

#[derive(Debug)]
pub struct Plugboard {
    wiring: HashMap<char, char>,
}

impl Plugboard {
    pub fn new(letter_mapping: &str) -> Self {
        Plugboard {
            wiring: generate_wiring_hashmap(letter_mapping),
        }
    }

    // TODO: Implement this
    pub fn encode(&self, index: u8) -> u8 {
        index
    }
}

fn generate_wiring_hashmap(letter_mapping: &str) -> HashMap<char, char> {
    let mut hashmap: HashMap<char, char> = HashMap::new();

    if letter_mapping.len() < 2 {
        return hashmap;
    }
    let string_bytes: &[u8] = letter_mapping.as_bytes();

    let mut index: usize = 0;

    while index < string_bytes.len() - 1 {
        hashmap.insert(string_bytes[index] as char, string_bytes[index + 1] as char);
        index += 2;
    }
    //println!("Plugboard: {:?}, {:?}", hashmap, string_bytes);
    return hashmap;
}
