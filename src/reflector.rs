use crate::charindex::alphabet_string_to_u8_array;

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

    pub fn encode(&self, index: u8) -> u8 {
        self.wiring[index as usize]
    }
}

fn get_wiring(name: &str) -> [u8; 26] {
    match name {
        "B" => alphabet_string_to_u8_array("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
        "C" => alphabet_string_to_u8_array("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        _ => panic!("Wiring for {} Reflector not found", name),
    }
}
