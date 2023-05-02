use std::{
    fs::OpenOptions,
    io::{self, stdout, Write},
};

use stats::{io_utils::read_tallies, mapping};

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
    // The table is something like this
    //
    //               | Absorb | Scatter | Fission | Collision | Census | NumSeg
    // CycleTracking | ...
    //
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tracking.dat")
        .unwrap();
    writeln!(file, ",Absorb,Scatter,Fission,Collision,Census,NumSeg").unwrap();
    // write correlation coeffs
    writeln!(
        file,
        "CycleTracking, {:.5}, {:.5}, {:.5}, {:.5}, {:.5}, {:.5}",
        tracking_res[0],
        tracking_res[1],
        tracking_res[2],
        tracking_res[3],
        tracking_res[4],
        tracking_res[5],
    )
    .unwrap();
    // padding values for it to be considered a matrix
    writeln!(file, "Dummy, 0, 0, 0, 0, 0, 0").unwrap();
}

pub fn save_popsync_results(popsync_res: &[f64]) {
    // The table is something like this
    //
    //                   | Source | Rr | Split
    // PopulationControl | ...
    // CycleSync         | ...
    //
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("popsync.dat")
        .unwrap();
    writeln!(file, ",Source,Rr,Split").unwrap();
    writeln!(
        file,
        "CycleSync, {:.5}, {:.5}, {:.5}",
        popsync_res[0], popsync_res[1], popsync_res[2]
    )
    .unwrap();
    writeln!(
        file,
        "PopulationControl, {:.5}, {:.5}, {:.5}",
        popsync_res[3], popsync_res[4], popsync_res[5]
    )
    .unwrap();
}
