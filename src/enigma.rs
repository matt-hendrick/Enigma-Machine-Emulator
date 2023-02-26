use crate::charindex::{char_to_index, index_to_char};
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

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
        rotor_types: [u8; 3],
        rotor_positions: [u8; 3],
        ring_settings: [u8; 3],
        reflector: &str,
        plugboard_mapping: &str,
    ) -> Self {
        Enigma {
            right_rotor: Rotor::new(rotor_types[0], rotor_positions[0], ring_settings[0]),
            middle_rotor: Rotor::new(rotor_types[1], rotor_positions[1], ring_settings[1]),
            left_rotor: Rotor::new(rotor_types[2], rotor_positions[2], ring_settings[2]),
            reflector: Reflector::new(reflector),
            plugboard: Plugboard::new(plugboard_mapping),
        }
    }

    fn rotate(&mut self) {
        if self.middle_rotor.is_at_notch() {
            self.middle_rotor.turn_rotor();
            self.left_rotor.turn_rotor();
        } else if self.right_rotor.is_at_notch() {
            self.middle_rotor.turn_rotor();
        }

        // right always turns
        self.right_rotor.turn_rotor();
    }

    pub fn encrypt_string(&mut self, raw_string: String) -> String {
        let original_text = raw_string.to_uppercase();
        let mut encrypted_text: String = String::new();

        for char in original_text.chars() {
            if !char.is_ascii_uppercase() {
                continue;
            } else {
                println!("{}, {:?}", char, encrypted_text);
                let result = self.encrypt_char(char);
                encrypted_text.push(index_to_char(result));
            }
        }
        encrypted_text
    }

    fn encrypt_char(&mut self, letter: char) -> u8 {
        self.rotate();
        let letter_char_index: u8 = char_to_index(letter);

        // goes into plugboard
        let c1 = self.plugboard.encode(letter_char_index);

        // goes through each rotors forward wiring
        // rotor R F
        let c2 = self.right_rotor.forward_encode(c1);
        // rotor M F
        let c3 = self.middle_rotor.forward_encode(c2);
        // rotor L F
        let c4 = self.left_rotor.forward_encode(c3);

        // reflector
        let c5 = self.reflector.encode(c4);

        // goes through backward wirings
        // rotor L B
        let c6 = self.left_rotor.backward_encode(c5);
        // rotor M B
        let c7 = self.middle_rotor.backward_encode(c6);
        // rotor R B
        let c8 = self.right_rotor.backward_encode(c7);

        // goes into plugboard
        let c9 = self.plugboard.encode(c8);

        //output
        c9
    }
}
