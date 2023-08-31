use std::nostr::{Event, Relay, Keys};
use std::lightning::{Balance, Channel, Invoice};

use nostr::{Event, Npub, Relay, Event};
use lightning::{Balace, Channel, HashPayment};
use lightning::{Open channel, Close channel};


fn main() {
    let matches = App::new("Rust CLI Template")
        .version("0.1.0")
        .author("Your Name")
        .about("A template CLI written in Rust")
        .arg(Arg::new("input")
            .about("Input file")
            .required(true)
            .index(1))
        .arg(Arg::new("output")
            .about("Output file")
            .required(true)
            .index(2))
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();

    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);
