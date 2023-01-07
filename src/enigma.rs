// mod plugboard;
// mod reflector;
// mod rotor;
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

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
        // TODO: break this out as a function and validate input

        // TODO: Finish the below. Current implementation is broken
        let parsed_rotor_types = rotor_types.as_bytes();
        let parsed_rotor_positions = rotor_positions.as_bytes();
        let parsed_ring_settings = ring_settings.as_bytes();
        Enigma {
            leftRotor: Rotor::new(
                parsed_rotor_types[0] as i32,
                parsed_rotor_positions[0] as i32,
                parsed_ring_settings[0] as i32,
            ),
            middleRotor: Rotor::new(
                parsed_rotor_types[1] as i32,
                parsed_rotor_positions[1] as i32,
                parsed_ring_settings[1] as i32,
            ),
            rightRotor: Rotor::new(
                parsed_rotor_types[2] as i32,
                parsed_rotor_positions[2] as i32,
                parsed_ring_settings[2] as i32,
            ),
            reflector: Reflector::new(reflector),
            plugboard: Plugboard::new(plugboard_mapping),
        }
    }
}
