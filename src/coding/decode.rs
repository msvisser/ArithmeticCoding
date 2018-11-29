use super::{Probability, Range};
use super::{HALF, QUARTER, SYMBOLS, PRECISION, EOF};

pub struct ACDecoder<I> {
    iter: I,

    finished: bool,

    range: Range,
    fraction: u64,

    probability: Probability,
}

impl<I> ACDecoder<I> where I: Iterator<Item = bool> {
    pub fn new(iter: I) -> Self {
        let probs = vec![1; 257];

        let mut decoder = Self {
            iter: iter,

            finished: false,

            range: Range::new(),
            fraction: 0,

            probability: Probability::new(probs)
        };
        decoder.initial_fraction();
        decoder
    }

    fn initial_fraction(&mut self) {
        for i in 0..PRECISION {
            if let Some(true) = self.iter.next() {
                self.fraction += 1 << (PRECISION - (i + 1));
            }
        }
    }

    fn increment_fraction(&mut self) {
        if let Some(true) = self.iter.next() {
            self.fraction += 1;
        }
    }
}

impl<I> Iterator for ACDecoder<I> where I: Iterator<Item = bool> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut result: Option<u8> = None;
        for symbol in 0..SYMBOLS {
            let new_range = self.range.select_symbol(symbol, &self.probability);

            if new_range.contains(self.fraction) {
                if symbol == EOF {
                    self.finished = true;
                    return None;
                }

                self.range = new_range;
                self.probability.increment(symbol);
                result = Some(symbol as u8);
                break;
            }
        }

        while self.range.in_bottom_half() || self.range.in_upper_half() {
            if self.range.in_bottom_half() {
                self.range.scale_bottom_half();
                self.fraction *= 2;
            } else if self.range.in_upper_half() {
                self.range.scale_upper_half();
                self.fraction = 2 * (self.fraction - HALF);
            }

            self.increment_fraction();
        }

        while self.range.in_middle_half() {
            self.range.scale_middle_half();
            self.fraction = 2 * (self.fraction - QUARTER);
            self.increment_fraction();
        }

        result
    }
}
