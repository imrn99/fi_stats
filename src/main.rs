use std::io::{self, stdout, Write};

use stats::{
    io_utils::{read_tallies, read_timers},
    mapping,
};

fn main() {
    // Input handling
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

    tallies_input = tallies_input.trim().to_owned();
    timers_input = timers_input.trim().to_owned();
    println!("Tallies file: \"{}\"", tallies_input);
    println!("Timers file: \"{}\"", timers_input);

    let tallies_data = read_tallies(&tallies_input);
    //let timers_data = read_timers(&timers_input);

    // Correlation analysis
    let tracking_res = mapping::build_tracking_results(&tallies_data);
    let popsync_res = mapping::build_popsync_results(&tallies_data);

    save_tracking_results(&tracking_res);
    save_popsync_results(&popsync_res);
}

pub fn save_tracking_results(tracking_res: &[f64]) {
    todo!()
}

pub fn save_popsync_results(popsync_res: &[f64]) {
    todo!()
}
