use std::fmt;

#[derive(Debug)]
pub struct Policy {
    pub min: u64,
    pub max: u64,
    pub governor: String
}

impl Policy {
    pub fn new(min: u64, max: u64, governor: &str) -> Policy {
        let mut result = Policy {
            min: min,
            max: max,
            governor: String::new()
        };

        result.governor.push_str(governor);

        result
    }
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Policy{{min: {}, max: {}, governor: {}}}",
               self.min, self.max, self.governor)
    }
}
