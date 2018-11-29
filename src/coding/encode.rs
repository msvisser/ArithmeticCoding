use super::{Probability, Range};
use super::{SYMBOLS, EOF};

pub struct ACEncoder<I> {
    iter: I,

    finished: bool,
    emit_buf: Vec<bool>,

    probability: Probability,
    range: Range,
    middle_count: usize,
}

impl<I> ACEncoder<I> where I: Iterator<Item = u8> {
    pub fn new(iter: I) -> Self {
        let probs = vec![1; SYMBOLS];

        Self {
            iter: iter,

            finished: false,
            emit_buf: Vec::new(),

            probability: Probability::new(probs),
            range: Range::new(),
            middle_count: 0,
        }
    }

    fn emit(&mut self, bit: bool) {
        self.emit_buf.push(bit);
        for _ in 0..self.middle_count {
            self.emit_buf.push(!bit);
        }
        self.middle_count = 0;
    }

    fn generate_for_symbol(&mut self, symbol: usize) {
        self.range = self.range.select_symbol(symbol, &self.probability);

        while self.range.in_bottom_half() || self.range.in_upper_half() {
            if self.range.in_bottom_half() {
                self.range.scale_bottom_half();
                self.emit(false);
            } else if self.range.in_upper_half() {
                self.range.scale_upper_half();
                self.emit(true);
            }
        }

        while self.range.in_middle_half() {
            self.middle_count += 1;
            self.range.scale_middle_half();
        }

        self.probability.increment(symbol);
    }

    fn finalize(&mut self) {
        self.middle_count += 1;
        if self.range.in_bottom_quarter() {
            self.emit(false);
        } else {
            self.emit(true);
        }
        self.finished = true;
    }
}

impl<I> Iterator for ACEncoder<I> where I: Iterator<Item = u8> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        while self.emit_buf.is_empty() {
            if self.finished {
                return None;
            }

            if let Some(symbol) = self.iter.next() {
                self.generate_for_symbol(symbol as usize);
            } else {
                self.generate_for_symbol(EOF);
                self.finalize();
            }
        }

        Some(self.emit_buf.remove(0))
    }
}
