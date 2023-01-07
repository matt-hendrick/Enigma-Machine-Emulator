mod rotor;
use rotor::Rotor;
use std::env::{args, Args};

fn main() {
    let args: Vec<String> = parse_args();

    println!("Args were: {:?}", args);

    let mut rotor: Rotor = Rotor::new("I", 2, 3, 3);

    println!("{}, {}", rotor.is_at_notch(), rotor.get_rotor_position());
    rotor.reset_rotor_position();
    println!("{}, {}", rotor.is_at_notch(), rotor.get_rotor_position());
}

fn parse_args() -> Vec<String> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        panic!("No args passed!")
    }

    return args;
}
