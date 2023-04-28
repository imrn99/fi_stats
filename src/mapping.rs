pub const N_EVENT_RV: usize = 9;

pub enum EventRV {
    Source = 2,
    Rr = 3,
    Split = 4,
    Absorb = 5,
    Scatter = 6,
    Fission = 7,
    Collision = 9,
    Census = 10,
    NumSeg = 11,
}

impl EventRV {
    pub fn get_csv_idx(idx: usize) -> usize {
        assert!(idx < N_EVENT_RV);
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
            _ => unreachable!(),
        }
    }
}

pub enum TimerSV {
    Main = 0,
    PopulationControl = 1,
    CycleTracking = 2,
    CycleSync = 5,
}
