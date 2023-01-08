use std::collections::HashMap;

#[derive(Debug)]
pub struct Rotor {
    rotor_type: i32,
    forward_wiring: Vec<usize>,
    inverse_wiring: Vec<usize>,
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
        wiring: Vec<char>,
        rotor_position: i32,
        notch_position: i32,
        ring_setting: i32,
    ) -> Self {
        let mut forward_wiring: Vec<usize> = encode_wiring(wiring);

        Rotor {
            rotor_type: rotor_type,
            inverse_wiring: reverse_wiring(&forward_wiring),
            forward_wiring: forward_wiring,
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
// Rotor mapping pulled from https://en.wikipedia.org/wiki/Enigma_rotor_details
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

fn char_to_index(letter: char) -> usize {
    assert!(letter.is_ascii_uppercase());
    letter as usize - 65
}

fn index_to_char(int: usize) -> char {
    assert!(int < 26);
    (int as u8 + 65) as char
}

fn encode_wiring(wiring: Vec<char>) -> Vec<usize> {
    let mut index_vec: Vec<usize> = Vec::new();

    for letter in wiring {
        index_vec.push(char_to_index(letter));
    }

    index_vec
}

fn reverse_wiring(forward_wiring: &Vec<usize>) -> Vec<usize> {
    let mut inverse_wiring: Vec<usize> = vec![0; 26];

    for (i, _) in forward_wiring.iter().enumerate() {
        inverse_wiring[forward_wiring[i]] = i;
    }

    inverse_wiring
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
