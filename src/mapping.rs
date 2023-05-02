use crate::variables::{correlation, FiniteDiscreteRV};

pub const N_EVENT_RV: usize = 9;
pub const N_TIMER_RV: usize = 3;

pub const POPSYNC_CORRELATIONS: [(EventRV, TimerRV); 6] = [
    (EventRV::Source, TimerRV::PopulationControl),
    (EventRV::Source, TimerRV::CycleSync),
    (EventRV::Rr, TimerRV::PopulationControl),
    (EventRV::Rr, TimerRV::CycleSync),
    (EventRV::Split, TimerRV::PopulationControl),
    (EventRV::Split, TimerRV::CycleSync),
];

pub const TRACKING_CORRELATIONS: [(EventRV, TimerRV); 6] = [
    (EventRV::Absorb, TimerRV::CycleTracking),
    (EventRV::Scatter, TimerRV::CycleTracking),
    (EventRV::Fission, TimerRV::CycleTracking),
    (EventRV::Collision, TimerRV::CycleTracking),
    (EventRV::Census, TimerRV::CycleTracking),
    (EventRV::NumSeg, TimerRV::CycleTracking),
];

pub enum EventRV {
    Source = 2,
    Rr = 3,
    Split = 4,
    Absorb = 5,
    Scatter = 6,
    Fission = 7,
    Collision = 9,
    Census = 11,
    NumSeg = 12,
}

pub enum TimerRV {
    PopulationControl = 14,
    CycleTracking = 15,
    CycleSync = 16,
}

pub fn get_csv_idx(idx: usize) -> usize {
    assert!(idx < N_EVENT_RV + N_TIMER_RV);
    match idx {
        0 => EventRV::Source as usize,
        1 => EventRV::Rr as usize,
        2 => EventRV::Split as usize,
        3 => EventRV::Absorb as usize,
        4 => EventRV::Scatter as usize,
        5 => EventRV::Fission as usize,
        6 => EventRV::Collision as usize,
        7 => EventRV::Census as usize,
        8 => EventRV::NumSeg as usize,
        9 => TimerRV::PopulationControl as usize,
        10 => TimerRV::CycleTracking as usize,
        11 => TimerRV::CycleSync as usize,
        _ => unreachable!(),
    }
}

pub fn get_table_idx(idx: usize) -> usize {
    match idx {
        2 => 0,
        3 => 1,
        4 => 2,
        5 => 3,
        6 => 4,
        7 => 5,
        9 => 6,
        11 => 7,
        12 => 8,
        14 => 9,
        15 => 10,
        16 => 11,
        _ => unreachable!(),
    }
}

pub enum TimerSV {
    Main = 0,
    PopulationControl = 1,
    CycleTracking = 2,
    CycleSync = 5,
}

pub fn build_tracking_results(tallies_data: &[FiniteDiscreteRV]) -> Vec<f64> {
    // The table is something like this
    //
    //               | Absorb | Scatter | Fission | Collision | Census | NumSeg
    // CycleTracking | ...
    //

    vec![
        correlation(
            &tallies_data[get_table_idx(EventRV::Absorb as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleTracking as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Scatter as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleTracking as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Fission as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleTracking as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Collision as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleTracking as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Census as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleTracking as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::NumSeg as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleTracking as usize)],
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
            &tallies_data[get_table_idx(EventRV::Source as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleSync as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Rr as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleSync as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Split as usize)],
            &tallies_data[get_table_idx(TimerRV::CycleSync as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Source as usize)],
            &tallies_data[get_table_idx(TimerRV::PopulationControl as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Rr as usize)],
            &tallies_data[get_table_idx(TimerRV::PopulationControl as usize)],
        ),
        correlation(
            &tallies_data[get_table_idx(EventRV::Split as usize)],
            &tallies_data[get_table_idx(TimerRV::PopulationControl as usize)],
        ),
    ]
}
