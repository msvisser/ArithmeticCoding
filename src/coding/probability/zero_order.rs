use super::Probability;

pub struct ProbabilityZeroOrder {
    cumulative: Vec<u64>,
}

impl ProbabilityZeroOrder {
    pub fn new(probabilities: &Vec<u64>) -> Self {
        let symbols = probabilities.len();

        // Create a list of cumulative probabilities (CDF)
        let mut cumulative = Vec::new();
        // Cumulative probability starts off at zero
        cumulative.push(0);
        // Create the cumulative values for all the symbols
        for symbol in 0..symbols {
            let previous_cumulative = cumulative.get(symbol).unwrap_or(&0);
            let previous_probability = probabilities.get(symbol).unwrap_or(&0);
            cumulative.push(previous_probability + previous_cumulative);
        }

        Self {
            cumulative: cumulative,
        }
    }
}

impl Probability for ProbabilityZeroOrder {
    fn cumulative_bottom(&self, symbol: usize) -> u64 {
        // Return the cumulative probability excluding the symbol
        self.cumulative[symbol]
    }

    fn cumulative_top(&self, symbol: usize) -> u64 {
        // Return the cumulative probability including the symbol
        self.cumulative[symbol + 1]
    }

    fn total(&self) -> u64 {
        // Return the total of all probabilities
        *self.cumulative.last().unwrap()
    }

    fn increment(&mut self, symbol: usize) {
        // Increment the probability of a certain symbol
        for i in (symbol + 1)..self.cumulative.len() {
            self.cumulative[i] += 1;
        }
    }

    fn update_last(&mut self, _symbol: usize) {}
}
