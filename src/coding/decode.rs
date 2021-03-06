use super::{Probability, Range};
use super::{HALF, QUARTER, PRECISION};

pub struct ACDecoder<I> {
    iter: I,
    finished: bool,

    probability: Box<Probability>,
    range: Range,
    fraction: u64,
}

impl<I> ACDecoder<I> where I: Iterator<Item = bool> {
    // Create a new arithmetic decoder
    pub fn new(iter: I, probability: Box<Probability>) -> Self {
        let mut decoder = Self {
            iter: iter,
            finished: false,

            probability: probability,
            range: Range::new(),
            fraction: 0,
        };
        // Set-up the initial values of the fraction
        decoder.initial_fraction();
        decoder
    }

    fn initial_fraction(&mut self) {
        // Load the initial value for the fraction from the bitstream
        for i in 0..PRECISION {
            if let Some(true) = self.iter.next() {
                self.fraction += 1 << (PRECISION - (i + 1));
            }
        }
    }

    fn increment_fraction(&mut self) {
        // Update the fraction with a new bit
        if let Some(true) = self.iter.next() {
            self.fraction += 1;
        }
    }

    fn decode_symbol(&mut self, symbols: usize) -> Option<usize> {
        let mut result: Option<usize> = None;
        // Try to find a symbol which matches the fraction
        for symbol in 0..symbols {
            // Generate the range for the possible symbol
            let new_range = self.range.select_symbol(symbol, &*self.probability);

            // Check that this new range contains the fraction
            if new_range.contains(self.fraction) {
                // If the encoded symbol is an EOF finish decoding
                if symbol == symbols - 1 {
                    self.finished = true;
                    return None;
                }

                // Update the current range
                self.range = new_range;
                // Increment the probability for this symbol, to stay in sync
                // with the encoder
                self.probability.increment(symbol);
                self.probability.update_last(symbol);

                result = Some(symbol);
                break;
            }
        }

        // Rescale the current range when in the bottom or top half
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

        // Rescale the range when in the middle half
        while self.range.in_middle_half() {
            self.range.scale_middle_half();
            self.fraction = 2 * (self.fraction - QUARTER);
            self.increment_fraction();
        }

        result
    }
}

impl<I> Iterator for ACDecoder<I> where I: Iterator<Item = bool> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // When finished always return None
        if self.finished {
            return None;
        }

        // Return the decoded symbol
        match self.decode_symbol(257) {
            Some(symbol) => Some(symbol as u8),
            None => None,
        }
    }
}
