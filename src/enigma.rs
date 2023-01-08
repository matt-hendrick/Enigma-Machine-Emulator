// mod plugboard;
// mod reflector;
// mod rotor;
use crate::charindex::char_to_index;
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

// Helpful 3d walkthrough of the machine's mechanics - https://www.youtube.com/watch?v=ybkkiGtJmkM
#[derive(Debug)]
pub struct Enigma {
    leftRotor: Rotor,
    middleRotor: Rotor,
    rightRotor: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(
        rotor_types: &str,
        rotor_positions: &str,
        ring_settings: &str,
        reflector: &str,
        plugboard_mapping: &str,
    ) -> Self {
        // TODO: Parse args and pass into Enigma constructor to allow customizing configuration

        Enigma {
            leftRotor: Rotor::new(1, 0, 0),
            middleRotor: Rotor::new(2, 3, 4),
            rightRotor: Rotor::new(3, 4, 5),
            reflector: Reflector::new(reflector),
            plugboard: Plugboard::new(plugboard_mapping),
        }
    }

    pub fn forward(&self, letter: char) -> u8 {
        let index = char_to_index(letter);
        self.leftRotor.encode(index, true)
    }

    pub fn backward(&self, letter: char) -> u8 {
        let index = char_to_index(letter);
        self.leftRotor.encode(index, false)
    }
}
