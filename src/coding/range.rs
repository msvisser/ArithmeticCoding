use super::Probability;
use super::{WHOLE, HALF, QUARTER};

pub struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    pub fn new() -> Self {
        Self {
            lower: 0,
            upper: WHOLE,
        }
    }

    pub fn select_symbol(&mut self, symbol: usize, probability: &Probability) -> Self {
        let width = self.upper - self.lower;
        Self {
            lower: self.lower + width * probability.cumulative_bottom(symbol) / probability.total(),
            upper: self.lower + width * probability.cumulative_top(symbol) / probability.total(),
        }
    }

    pub fn in_bottom_half(&self) -> bool {
        self.upper < HALF
    }
    pub fn scale_bottom_half(&mut self) {
        self.lower *= 2;
        self.upper *= 2;
    }

    pub fn in_upper_half(&self) -> bool {
        self.lower > HALF
    }
    pub fn scale_upper_half(&mut self) {
        self.lower = 2 * (self.lower - HALF);
        self.upper = 2 * (self.upper - HALF);
    }

    pub fn in_middle_half(&self) -> bool {
        self.lower > QUARTER && self.upper < 3 * QUARTER
    }
    pub fn scale_middle_half(&mut self) {
        self.lower = 2 * (self.lower - QUARTER);
        self.upper = 2 * (self.upper - QUARTER);
    }

    pub fn in_bottom_quarter(&self) -> bool {
        self.lower <= QUARTER
    }

    pub fn contains(&self, value: u64) -> bool {
        self.lower <= value && value < self.upper
    }
}
