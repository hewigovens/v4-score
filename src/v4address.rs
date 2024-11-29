use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct V4AddressResult {
    pub salt: String,
    pub address: String,
    pub score: u32,
}

impl Ord for V4AddressResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for V4AddressResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for V4AddressResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Salt: {}, Addr: {}, Score: {}",
            self.salt, self.address, self.score
        )
    }
}
