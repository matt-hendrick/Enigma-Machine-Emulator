use crate::charindex::alphabet_string_to_u8_array;

#[derive(Debug, Clone)]
pub struct Reflector {
    name: String,
    wiring: [u8; 26],
}

impl Reflector {
    pub fn new(name: &str) -> Self {
        Reflector {
            name: name.to_string(),
            wiring: get_wiring(name),
        }
    }

    pub fn encode(&self, index: u8) -> u8 {
        self.wiring[index as usize]
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn get_wiring(name: &str) -> [u8; 26] {
    match name {
        "B" => alphabet_string_to_u8_array("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
        "C" => alphabet_string_to_u8_array("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        _ => panic!("Wiring for {} Reflector not found", name),
    }
}
