// mod plugboard;
// mod reflector;
// mod rotor;
use crate::charindex::{char_to_index, index_to_char};
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

// Helpful 3d walkthrough of the machine's mechanics - https://www.youtube.com/watch?v=ybkkiGtJmkM
#[derive(Debug)]
pub struct Enigma {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,
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
            right_rotor: Rotor::new(1, 0, 0),
            middle_rotor: Rotor::new(2, 0, 0),
            left_rotor: Rotor::new(3, 0, 0),
            reflector: Reflector::new(reflector),
            plugboard: Plugboard::new(plugboard_mapping),
        }
    }

    fn rotate(&mut self) {
        if self.middle_rotor.is_at_notch() {
            println!("Turned middle and left");
            self.middle_rotor.turn_rotor();
            self.left_rotor.turn_rotor();
        } else if self.right_rotor.is_at_notch() {
            println!("Turned middle");
            self.middle_rotor.turn_rotor();
        }

        // right always turns
        println!("Turned right");
        self.right_rotor.turn_rotor();
    }

    pub fn encrypt(&mut self, letter: char) -> u8 {
        self.rotate();
        let letter_char_index: u8 = char_to_index(letter);

        // goes into plugboard
        let c1 = self.plugboard.encode(letter_char_index);

        // goes through each rotors forward wiring
        // rotor R F
        let c2 = self.right_rotor.encode(c1, true);
        // rotor M F
        let c3 = self.middle_rotor.encode(c2, true);
        // rotor L F
        let c4 = self.left_rotor.encode(c3, true);

        // reflector
        let c5 = self.reflector.encode(c4);

        // goes through backward wirings
        // rotor L B
        let c6 = self.left_rotor.encode(c5, false);
        // rotor M B
        let c7 = self.middle_rotor.encode(c6, false);
        // rotor R B
        let c8 = self.right_rotor.encode(c7, false);

        // goes into plugboard
        let c9 = self.plugboard.encode(c8);

        //output

        c9
    }

    pub fn forward(&self, letter: char) {
        let index = char_to_index(letter);
        println!(
            "Left forward {} = {}, {}",
            letter,
            self.left_rotor.encode(index, true),
            index_to_char(self.left_rotor.encode(index, true))
        );
        println!(
            "Middle forward {} = {}, {}",
            letter,
            self.middle_rotor.encode(index, true),
            index_to_char(self.middle_rotor.encode(index, true))
        );
        println!(
            "Right forward {} = {}, {}",
            letter,
            self.right_rotor.encode(index, true),
            index_to_char(self.right_rotor.encode(index, true))
        );
    }

    pub fn backward(&self, letter: char) {
        let index = char_to_index(letter);
        println!(
            "Left backward {} = {}, {}",
            letter,
            self.left_rotor.encode(index, false),
            index_to_char(self.left_rotor.encode(index, false))
        );
        println!(
            "Middle backward {} = {}, {}",
            letter,
            self.middle_rotor.encode(index, false),
            index_to_char(self.middle_rotor.encode(index, false))
        );
        println!(
            "Right backward {} = {}, {}",
            letter,
            self.right_rotor.encode(index, false),
            index_to_char(self.right_rotor.encode(index, false))
        );
    }
}
