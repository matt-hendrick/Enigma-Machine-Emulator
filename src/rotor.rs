pub struct Rotor<'a> {
    name: &'a str,
    forward_wiring: &'a str,
    inverse_wiring: String,
    rotor_position: i32,
    notch_position: i32,
    ring_setting: i32,
}

impl<'a> Rotor<'a> {
    pub fn new(name: &'a str, rotor_position: i32, notch_position: i32, ring_setting: i32) -> Self {
        let wiring = get_wiring(name);
        return Self::init(name, wiring, rotor_position, notch_position, ring_setting);
    }

    fn init(
        name: &'a str,
        forward_wiring: &'a str,
        rotor_position: i32,
        notch_position: i32,
        ring_setting: i32,
    ) -> Self {
        Rotor {
            name: name,
            forward_wiring: forward_wiring,
            inverse_wiring: reverse_wiring(&forward_wiring),
            rotor_position: rotor_position,
            notch_position: notch_position,
            ring_setting: ring_setting,
        }
    }

    pub fn is_at_notch(&self) -> bool {
        return self.notch_position == self.rotor_position;
    }

    pub fn reset_rotor_position(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % 26;
    }

    pub fn get_rotor_position(&self) -> i32 {
        return self.rotor_position;
    }
}

fn reverse_wiring(forward_wiring: &str) -> String {
    let inverse_wiring: String = forward_wiring.chars().rev().collect::<String>();

    // TODO: Add whatever logic is needed to get the inverse wiring (it does not appear to just be the chars reversed)
    println!("{}", inverse_wiring);
    return inverse_wiring;
}

fn get_wiring(name: &str) -> &str {
    match name {
        "I" => "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
        "II" => "AJDKSIRUXBLHWTMCQGZNPYFVOE",
        "III" => "BDFHJLCPRTXVZNYEIWGAKMUSQO",
        _ => panic!(
            "No wiring mapping found for a Rotor under the name {}",
            name
        ),
    }
}
