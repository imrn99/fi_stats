use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::structures::{
    FiniteDiscreteRV, SummarizedVariable, TimerReport, TimerSV, N_TALLIED_DATA,
};

pub fn read_tallies(file_name: &str) -> [FiniteDiscreteRV; N_TALLIED_DATA] {
    let file = File::open(file_name).unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut values: [Vec<f64>; N_TALLIED_DATA] = [
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
    ];
    // for each line
    for result in reader.records() {
        let mut record = result.unwrap();
        record.trim();
        // for each column
        (0..N_TALLIED_DATA).for_each(|idx| {
            let val = record.get(idx).unwrap();
            values[idx].push(val.parse().unwrap())
        })
    }
    // convert value vectors to our structure
    values.map(|val| FiniteDiscreteRV::new(&val))
}

pub fn read_timers(file_name: &str) -> TimerReport {
    let mut res = [SummarizedVariable::default(); 6];
    let file = File::open(file_name).unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    // for each line
    for (timer_idx, result) in reader.records().enumerate() {
        let mut record = result.unwrap();
        record.trim();
        // lmao
        res[timer_idx].lowest = record.get(2).unwrap().parse().unwrap();
        res[timer_idx].mean = record.get(3).unwrap().parse().unwrap();
        res[timer_idx].highest = record.get(4).unwrap().parse().unwrap();
        res[timer_idx].total = record.get(5).unwrap().parse().unwrap();
    }

    TimerReport { timers_data: res }
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
    writeln!(file, ",Rr,Split").unwrap();
    writeln!(
        file,
        "CycleSync, {:.5}, {:.5}",
        popsync_res[1], popsync_res[2]
    )
    .unwrap();
    writeln!(
        file,
        "PopulationControl, {:.5}, {:.5}",
        popsync_res[4], popsync_res[5]
    )
    .unwrap();
}

pub fn compile_scaling_data(timer_data: &[TimerReport], n_start: usize, step: usize) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("scaling.dat")
        .unwrap();
    // we assume correct ordering of the summarized variables
    // i.e. lowest number of particle to highest
    // also assume an arithmetic progression for n_particles
    writeln!(
        file,
        "n_particles,PopulationControlAvg,CycleTrackingAvg,CycleSyncAvg"
    )
    .unwrap();
    timer_data.iter().enumerate().for_each(|(idx, report)| {
        writeln!(
            file,
            "{},{},{},{}",
            n_start + idx * step,
            report[TimerSV::PopulationControl].mean,
            report[TimerSV::CycleTracking].mean,
            report[TimerSV::CycleSync].mean,
        )
        .unwrap();
    });
}
