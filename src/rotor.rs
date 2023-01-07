#[derive(Debug)]
pub struct Rotor {
    rotor_type: i32,
    forward_wiring: Vec<char>,
    inverse_wiring: Vec<char>,
    rotor_position: i32,
    notch_position: i32,
    ring_setting: i32,
}

impl Rotor {
    pub fn new(rotor_type: i32, rotor_position: i32, ring_setting: i32) -> Self {
        let wiring = get_wiring(rotor_type);
        let notch_position = get_notch_position(rotor_type);
        return Self::init(
            rotor_type,
            wiring,
            rotor_position,
            notch_position,
            ring_setting,
        );
    }

    fn init(
        rotor_type: i32,
        forward_wiring: Vec<char>,
        rotor_position: i32,
        notch_position: i32,
        ring_setting: i32,
    ) -> Self {
        // TODO: It appears that Enigma inverse wirings aren't just the reverse of the forwarding wiring. Figure that out
        // Can also either remove the init function or include the get_wiring_table call here
        let mut inverse_wiring: Vec<char> = forward_wiring.to_vec();
        inverse_wiring.reverse();

        Rotor {
            rotor_type: rotor_type,
            forward_wiring: forward_wiring,
            inverse_wiring: inverse_wiring,
            rotor_position: rotor_position,
            notch_position: notch_position,
            ring_setting: ring_setting,
        }
    }

    pub fn is_at_notch(&self) -> bool {
        return self.notch_position == self.rotor_position;
    }

    pub fn increment_rotor_position(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % 26;
    }

    pub fn get_rotor_position(&self) -> i32 {
        return self.rotor_position;
    }

    // TODO: Implement this
    pub fn encode(index: i32) {}
}

// TODO: Consider replacing the below with Enums
// TODO: Add the additional rotors
fn get_wiring(rotor_type: i32) -> Vec<char> {
    match rotor_type {
        1 => "EKMFLGDQVZNTOWYHXUSPAIBRCJ".chars().collect(),
        2 => "AJDKSIRUXBLHWTMCQGZNPYFVOE".chars().collect(),
        3 => "BDFHJLCPRTXVZNYEIWGAKMUSQO".chars().collect(),
        _ => panic!(
            "No wiring mapping found for a Rotor under the rotor_type {}",
            rotor_type
        ),
    }
}

fn get_notch_position(rotor_type: i32) -> i32 {
    match rotor_type {
        1 => 16,
        2 => 4,
        3 => 21,
        _ => panic!(
            "No notch position found for a Rotor under the rotor_type {}",
            rotor_type
        ),
    }
}
