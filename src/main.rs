use std::io::{self, stdout, Write};

use stats::{
    io_utils::{read_tallies, save_popsync_results, save_tracking_results},
    mapping,
};

fn main() {
    // Input handling
    let mut txt_input = String::new();

    println!();
    println!("What do we do?");

    while (txt_input.trim() != "y") & (txt_input.trim() != "n") {
        txt_input.clear();
        print!("Version comparison? (y/n): ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
    }
    let comparison = txt_input.trim() == "y";

    txt_input.clear();
    while (txt_input.trim() != "y") & (txt_input.trim() != "n") {
        txt_input.clear();
        print!("Correlation study? (y/n): ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
    }
    let correlation = txt_input.trim() == "y";

    txt_input.clear();
    while (txt_input.trim() != "y") & (txt_input.trim() != "n") {
        txt_input.clear();
        print!("Scaling study? (y/n): ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
    }
    let scaling = txt_input.trim() == "y";

    txt_input.clear();

    if comparison {
        println!("+---------------------------------------+");
        println!("|Performance Comparison Between Versions|");
        // Get old report file
        print!("Old timers report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!();
        let old_timers = txt_input.trim().to_owned();
        txt_input.clear();
        // Get new report file
        print!("New timers report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!();
        let new_timers = txt_input.trim().to_owned();
        txt_input.clear();

        // Get data, process it, save results
        // ...
    }

    if correlation {
        println!("+-----------------+");
        println!("|Correlation Study|");
        // Get tallied data
        print!("Tallies report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!();
        let tallies_report = txt_input.trim().to_owned();
        txt_input.clear();

        // Get data, process it, save results
        let tallies_data = read_tallies(&tallies_report);
        let tracking_res = mapping::build_tracking_results(&tallies_data);
        let popsync_res = mapping::build_popsync_results(&tallies_data);
        save_tracking_results(&tracking_res);
        save_popsync_results(&popsync_res);
    }

    if scaling {}

    /*
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
    println!("Timers file: \"{}\"", timers_input);*/

    //let tallies_data = read_tallies(&tallies_input);
    //let timers_data = read_timers(&timers_input);

    // Correlation analysis
    //let tracking_res = mapping::build_tracking_results(&tallies_data);
    //let popsync_res = mapping::build_popsync_results(&tallies_data);

    //save_tracking_results(&tracking_res);
    //save_popsync_results(&popsync_res);
}
