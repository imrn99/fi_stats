use std::io::{self, stdout, Write};

use stats::io_utils::read_tallies;

fn main() {
    let mut tallies_input = String::new();
    let mut timers_input = String::new();

    println!();
    println!("Tallies report .csv file: ");
    stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut tallies_input)
        .expect("Problem reading input.");

    println!("Timers report .csv file: ");
    stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut timers_input)
        .expect("Problem reading input.");

    let tallies_data = read_tallies(&tallies_input);
    let timers_data = read_tallies(&timers_input);
}
