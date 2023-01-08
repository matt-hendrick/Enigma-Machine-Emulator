use crate::charindex::{char_to_index, index_to_char};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rotor {
    rotor_type: u8,
    forward_wiring: [u8; 26],
    backward_wiring: [u8; 26],
    rotor_position: u8,
    notch_position: u8,
    ring_setting: u8,
}

impl Rotor {
    pub fn new(rotor_type: u8, rotor_position: u8, ring_setting: u8) -> Self {
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
        rotor_type: u8,
        wiring: Vec<char>,
        rotor_position: u8,
        notch_position: u8,
        ring_setting: u8,
    ) -> Self {
        let mut forward_wiring: [u8; 26] = encode_wiring(wiring);

        Rotor {
            rotor_type: rotor_type,
            backward_wiring: reverse_wiring(&forward_wiring),
            forward_wiring: forward_wiring,
            rotor_position: rotor_position,
            notch_position: notch_position,
            ring_setting: ring_setting,
        }
    }

    pub fn is_at_notch(&self) -> bool {
        self.notch_position == self.rotor_position
    }

    pub fn increment_rotor_position(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % 26;
    }

    pub fn get_rotor_position(&self) -> u8 {
        self.rotor_position
    }

    pub fn encode(&self, index: u8, is_forward: bool) -> u8 {
        let offset: u8 = (26 + self.rotor_position - self.ring_setting);
        let mut result: u8 = 0;
        if is_forward {
            result = (self.forward_wiring[((index + offset) % 26) as usize])
        } else {
            result = (self.backward_wiring[((index + offset) % 26) as usize])
        }
        println!(
            "{}, {} encoded = {}, {}",
            index,
            index_to_char(index),
            result,
            index_to_char(result)
        );
        result
    }
}

// TODO: Consider replacing the below with Enums
// TODO: Add the additional rotors
// Rotor mapping pulled from https://en.wikipedia.org/wiki/Enigma_rotor_details
fn get_wiring(rotor_type: u8) -> Vec<char> {
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

fn encode_wiring(wiring: Vec<char>) -> [u8; 26] {
    let mut index_array: [u8; 26] = [0; 26];
    for (i, letter) in wiring.iter().enumerate() {
        index_array[i] = char_to_index(*letter);
    }

    index_array
}

fn reverse_wiring(forward_wiring: &[u8; 26]) -> [u8; 26] {
    let mut backward_wiring: [u8; 26] = [0; 26];

    for (i, _) in forward_wiring.iter().enumerate() {
        backward_wiring[forward_wiring[i] as usize] = i as u8;
    }

    backward_wiring
}

fn get_notch_position(rotor_type: u8) -> u8 {
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
