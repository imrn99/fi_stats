use crate::structures::{correlation, FiniteDiscreteRV, TalliedData};

pub const POPSYNC_CORRELATIONS: [(TalliedData, TalliedData); 6] = [
    (TalliedData::Source, TalliedData::PopulationControl),
    (TalliedData::Source, TalliedData::CycleSync),
    (TalliedData::Rr, TalliedData::PopulationControl),
    (TalliedData::Rr, TalliedData::CycleSync),
    (TalliedData::Split, TalliedData::PopulationControl),
    (TalliedData::Split, TalliedData::CycleSync),
];

pub const TRACKING_CORRELATIONS: [(TalliedData, TalliedData); 6] = [
    (TalliedData::Absorb, TalliedData::CycleTracking),
    (TalliedData::Scatter, TalliedData::CycleTracking),
    (TalliedData::Fission, TalliedData::CycleTracking),
    (TalliedData::Collision, TalliedData::CycleTracking),
    (TalliedData::Census, TalliedData::CycleTracking),
    (TalliedData::NumSeg, TalliedData::CycleTracking),
];

pub fn build_tracking_results(tallies_data: &[FiniteDiscreteRV]) -> Vec<f64> {
    // The table is something like this
    //
    //               | Absorb | Scatter | Fission | Collision | Census | NumSeg
    // CycleTracking | ...
    //

    vec![
        correlation(
            &tallies_data[TalliedData::Absorb as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Scatter as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Fission as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Collision as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Census as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::NumSeg as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
    ]
}

pub fn build_popsync_results(tallies_data: &[FiniteDiscreteRV]) -> Vec<f64> {
    // The table is something like this
    //
    //                   | Source | Rr | Split
    // PopulationControl | ...
    // CycleSync         | ...
    //
    // gnuplot has the Y axis upside down, hence the vector:
    vec![
        correlation(
            &tallies_data[TalliedData::Source as usize],
            &tallies_data[TalliedData::CycleSync as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Rr as usize],
            &tallies_data[TalliedData::CycleSync as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Split as usize],
            &tallies_data[TalliedData::CycleSync as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Source as usize],
            &tallies_data[TalliedData::PopulationControl as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Rr as usize],
            &tallies_data[TalliedData::PopulationControl as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Split as usize],
            &tallies_data[TalliedData::PopulationControl as usize],
        ),
    ]
}
