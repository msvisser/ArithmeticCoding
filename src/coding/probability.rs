pub struct Probability {
    total: u64,
    cumulative: Vec<u64>,
}

impl Probability {
    pub fn new(probabilities: Vec<u64>) -> Self {
        let symbols = probabilities.len();
        let mut cumulative = Vec::new();
        cumulative.push(0);
        for symbol in 1..(symbols + 1) {
            let previous_cumulative = cumulative.get(symbol - 1).unwrap_or(&0);
            let previous_probability = probabilities.get(symbol - 1).unwrap_or(&0);
            cumulative.push(previous_probability + previous_cumulative);
        }

        Self {
            total: probabilities.iter().sum(),
            cumulative: cumulative,
        }
    }

    pub fn cumulative_bottom(&self, symbol: usize) -> u64 {
        self.cumulative[symbol]
    }

    pub fn cumulative_top(&self, symbol: usize) -> u64 {
        self.cumulative[symbol + 1]
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn increment(&mut self, symbol: usize) {
        self.total += 1;
        for i in (symbol + 1)..self.cumulative.len() {
            self.cumulative[i] += 1;
        }
    }
}
