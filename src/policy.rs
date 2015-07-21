

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
