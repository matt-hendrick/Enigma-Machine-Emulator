mod charindex;
mod enigma;
mod plugboard;
mod reflector;
mod rotor;
use enigma::Enigma;
use plugboard::Plugboard;
use rotor::Rotor;
use std::env::{args, Args};

fn main() {
    // let args: Vec<String> = parse_args();

    // println!("Args were: {:?}", args);

    // let mut rotor: Rotor = Rotor::new("I", 2, 3);

    // println!("{}, {}", rotor.is_at_notch(), rotor.get_rotor_position());
    // rotor.increment_rotor_position();
    // println!("{}, {}", rotor.is_at_notch(), rotor.get_rotor_position());

    // let mut plugboard: Plugboard = Plugboard::new("abcdefghij");

    let mut enigma: Enigma = Enigma::new("123", "123", "321", "B", "ABCDEFGHIJ");

    println!("Enigma: {:?}", enigma);

    assert!(enigma.forward('A') == 4);
    enigma.forward('A');
    enigma.backward('A');
}

fn parse_args() -> Vec<String> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        panic!("No args passed!")
    }

    return args;
}
