use super::{Probability, Range};

pub struct ACEncoder<I> {
    iter: I,

    finished: bool,
    emit_buf: Vec<bool>,

    probability: Box<Probability>,
    range: Range,
    middle_count: usize,
}

impl<I> ACEncoder<I> where I: Iterator<Item = u8> {
    // Create a new arithmetic encoder
    pub fn new(iter: I, probability: Box<Probability>) -> Self {
        Self {
            iter: iter,

            finished: false,
            emit_buf: Vec::new(),

            probability: probability,
            range: Range::new(),
            middle_count: 0,
        }
    }

    fn emit(&mut self, bit: bool) {
        // Emit a number of bits, this is always a bit, followed by a number of
        // opposite bits, depending on the times we were in the middle half
        self.emit_buf.push(bit);
        for _ in 0..self.middle_count {
            self.emit_buf.push(!bit);
        }
        self.middle_count = 0;
    }

    fn generate_for_symbol(&mut self, symbol: usize) {
        // Change the range to the sub-range for this symbol
        self.range = self.range.select_symbol(symbol, &*self.probability);

        // When in the bottom or top half we encode a zero or one
        while self.range.in_bottom_half() || self.range.in_upper_half() {
            if self.range.in_bottom_half() {
                // Rescale to the bottom half
                self.range.scale_bottom_half();
                // Emit a zero
                self.emit(false);
            } else if self.range.in_upper_half() {
                // Rescale to the upper half
                self.range.scale_upper_half();
                // Emit a one
                self.emit(true);
            }
        }

        // When in the middle half, rescale to the middle half and count
        while self.range.in_middle_half() {
            self.middle_count += 1;
            self.range.scale_middle_half();
        }

        // Increment the probability for this symbol
        self.probability.increment(symbol);
        self.probability.update_last(symbol);
    }

    fn finalize(&mut self) {
        // Finialize the encoding of the last symbol
        self.middle_count += 1;
        if self.range.in_bottom_quarter() {
            self.emit(false);
        } else {
            self.emit(true);
        }
        // Set finished
        self.finished = true;
    }
}

impl<I> Iterator for ACEncoder<I> where I: Iterator<Item = u8> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // As long as there are no bits to output, generate some more bits
        while self.emit_buf.is_empty() {
            // If we are finished return None
            if self.finished {
                return None;
            }

            if let Some(symbol) = self.iter.next() {
                // If there is a symbol left, generate bits for the symbol
                self.generate_for_symbol(symbol as usize);
            } else {
                // Otherwise generate an EOF and finalize the bits
                self.generate_for_symbol(256);
                self.finalize();
            }
        }

        // Return the first available bit
        Some(self.emit_buf.remove(0))
    }
}
