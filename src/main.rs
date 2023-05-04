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

    if scaling {
        println!("+-------------+");
        println!("|Scaling Study|");
        // Get tallied data
        println!("CURRENTLY ONLY SUPPORTS LINEAR SCALING");
        println!("GEOMETRIC PROGRESSION WILL BE ADDED IN THE FUTURE");
        println!("This study requires the input files to fit a pattern for easy reading.");
        println!("For example:");
        println!("|");
        println!("+--+>some_folder_with_data/");
        println!("|  |");
        println!("|  +--+timers_report10000.csv");
        println!("|     +timers_report20000.csv");
        println!("|     +timers_report30000.csv");
        println!("|     +timers_report40000.csv");
        println!("+--+>...");
        println!("In this case:");
        println!(" - the root is \"timers_report\".");
        println!(" - the starting number of particles is 10000");
        println!(" - the step is 10000");
        println!(" - the number of iteration is 4");
        println!();
        // Get naming root
        print!("Name root of the timers report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!();
        let root = txt_input.trim().to_owned();
        txt_input.clear();
        // get starting number of particles
        print!("Starting number of particles: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!();
        let n_start: usize = txt_input.parse().unwrap();
        txt_input.clear();
        // get step
        print!("Step: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!();
        let step: usize = txt_input.parse().unwrap();
        txt_input.clear();
        // get number of iterations
        print!("Number of iterations: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!();
        let n_iter: usize = txt_input.parse().unwrap();
        txt_input.clear();
    }
    //let tallies_data = read_tallies(&tallies_input);
    //let timers_data = read_timers(&timers_input);

    // Correlation analysis
    //let tracking_res = mapping::build_tracking_results(&tallies_data);
    //let popsync_res = mapping::build_popsync_results(&tallies_data);

    //save_tracking_results(&tracking_res);
    //save_popsync_results(&popsync_res);
}
