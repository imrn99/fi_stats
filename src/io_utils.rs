use std::fs::File;

use crate::{
    mapping::N_TALLIED_DATA,
    variables::{FiniteDiscreteRV, SummarizedVariable},
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

pub fn read_timers(file_name: &str) -> [SummarizedVariable; 6] {
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

    res
}
