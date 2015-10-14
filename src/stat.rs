use ::types::Frequency;
use std::fmt;


pub struct Stat {
    pub freq: Frequency,
    pub time_in_state: u64
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stat{{freq: {}, time_in_state: {}}}", self.freq, self.time_in_state)
    }
}
