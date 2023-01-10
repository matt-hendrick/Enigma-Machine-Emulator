use crate::charindex::{alphabet_string_to_u8_array, index_to_char};

#[derive(Debug)]
pub struct Reflector {
    wiring: [u8; 26],
}

impl Reflector {
    pub fn new(name: &str) -> Self {
        Reflector {
            wiring: get_wiring(name),
        }
    }

    // TODO: Implement this
    pub fn encode(&self, index: u8) -> u8 {
        let result: u8;
        result = self.wiring[index as usize];
        println!(
            "{}, {} reflected = {}, {}",
            index,
            index_to_char(index),
            result,
            index_to_char(result)
        );
        result
    }
}

// TODO: Probably can swap for Enums
fn get_wiring(name: &str) -> [u8; 26] {
    match name {
        "B" => alphabet_string_to_u8_array("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
        "C" => alphabet_string_to_u8_array("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        _ => panic!("Wiring for {} Reflector not found", name),
    }
}
