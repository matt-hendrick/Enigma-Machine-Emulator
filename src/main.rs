mod charindex;
mod enigma;
mod plugboard;
mod reflector;
mod rotor;
use clap::Parser;
use enigma::Enigma;
use plugboard::Plugboard;
use rotor::Rotor;
// use std::env::{args, Args};

use crate::charindex::index_to_char;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// text to encode
    #[arg(short = 'o')]
    original_text: String,

    /// rotor type
    #[arg(short = 't', default_value_t = ("123".to_string()))]
    rotor_types: String,

    /// rotor position
    #[arg(short = 'p', default_value_t = ("123".to_string()))]
    rotor_position: String,

    /// ring setting
    #[arg(short = 's', default_value_t = ("123".to_string()))]
    ring_setting: String,

    /// reflector
    #[arg(short = 'r', default_value_t = ("B".to_string()))]
    reflector: String,

    /// plugboard
    #[arg(short = 'b', default_value_t = ("".to_string()))]
    plugboard: String,
}

fn main() {
    let args = Args::parse();

    println!("args {:?}", args);

    let mut enigma: Enigma = Enigma::new("123", "123", "321", "B", "");

    let originalText = args.original_text.to_uppercase();
    println!("{}", originalText);
    let mut encodedText: String = String::new();

    let mut count = 0;
    for char in originalText.chars() {
        if char == ' ' {
            continue;
        }
        if count % 5 == 0 {
            encodedText.push(' ');
        }
        if !char.is_ascii_uppercase() {
            encodedText.push(char);
            println!("Added '{}'", char)
        } else {
            let result = enigma.encrypt(char);
            println!("Enigma encrypt A = {}, {}", result, index_to_char(result));
            encodedText.push(index_to_char(result));
        }
        count += 1;
    }

    println!(
        "Encrypted text = {}. Original Text = {}",
        encodedText, originalText
    );

    // println!("Enigma: {:?}", enigma)
    // let zResult = enigma.encrypt('Z');

    // println!("Enigma encrypt Z = {}, {}", zResult, index_to_char(zResult));

    // assert!(enigma.forward('A') == 4);
    // enigma.forward('A');
    // enigma.backward('A');
    // enigma.forward('Z');
    // enigma.backward('Z');
}

// fn parse_args() -> Vec<String> {
//     let args: Vec<String> = args().collect();

//     if args.len() == 1 {
//         panic!("No args passed!")
//     }

//     return args;
// }
