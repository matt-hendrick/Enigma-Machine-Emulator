#[derive(Debug)]
pub struct Reflector {
    wiring: Vec<char>,
}

impl Reflector {
    pub fn new(name: &str) -> Self {
        Reflector {
            wiring: get_wiring(name),
        }
    }

    // TODO: Implement this
    pub fn encode(index: u32) {}
}

// TODO: Probably can swap for Enums
fn get_wiring(name: &str) -> Vec<char> {
    match name {
        "B" => "YRUHQSLDPXNGOKMIEBFZCWVJAT".chars().collect(),
        "C" => "FVPJIAOYEDRZXWGCTKUQSBNMHL".chars().collect(),
        _ => panic!("Wiring for {} Reflector not found", name),
    }
}
