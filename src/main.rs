mod charindex;
mod enigma;
mod plugboard;
mod reflector;
mod rotor;
use enigma::Enigma;
use plugboard::Plugboard;
use rotor::Rotor;
use std::env::{args, Args};

use crate::charindex::index_to_char;

fn main() {
    let mut enigma: Enigma = Enigma::new("123", "123", "321", "B", "");

    println!("Enigma: {:?}", enigma);
    let aResult = enigma.encrypt('A');
    println!("Enigma encrypt A = {}, {}", aResult, index_to_char(aResult));
    // let zResult = enigma.encrypt('Z');

    // println!("Enigma encrypt Z = {}, {}", zResult, index_to_char(zResult));

    // assert!(enigma.forward('A') == 4);
    // enigma.forward('A');
    // enigma.backward('A');
    // enigma.forward('Z');
    // enigma.backward('Z');
}

fn parse_args() -> Vec<String> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        panic!("No args passed!")
    }

    return args;
}
