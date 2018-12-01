use super::{Probability, ProbabilityZeroOrder};

pub struct ProbabilityFirstOrder {
    cumulative: Vec<ProbabilityZeroOrder>,
    last_symbol: usize,
}

impl ProbabilityFirstOrder {
    pub fn new(probabilities: &Vec<u64>) -> Self {
        let symbols = probabilities.len();

        let mut all = Vec::new();

        for _ in 0..symbols {
            all.push(ProbabilityZeroOrder::new(probabilities));
        }

        Self {
            cumulative: all,
            last_symbol: symbols - 1,
        }
    }
}

impl Probability for ProbabilityFirstOrder {
    fn cumulative_bottom(&self, symbol: usize) -> u64 {
        // Return the cumulative probability excluding the symbol
        self.cumulative[self.last_symbol].cumulative_bottom(symbol)
    }

    fn cumulative_top(&self, symbol: usize) -> u64 {
        // Return the cumulative probability including the symbol
        self.cumulative[self.last_symbol].cumulative_top(symbol)
    }

    fn total(&self) -> u64 {
        // Return the total of all probabilities
        self.cumulative[self.last_symbol].total()
    }

    fn increment(&mut self, symbol: usize) {
        // Increment the probability of a certain symbol
        self.cumulative[self.last_symbol].increment(symbol);
    }

    fn update_last(&mut self, symbol: usize) {
        self.last_symbol = symbol;
    }
}
