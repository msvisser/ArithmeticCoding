pub struct BitPacker<I> {
    iter: I
}

impl<I> BitPacker<I> where I: Iterator<Item = bool> {
    pub fn new(iter: I) -> Self {
        Self { iter: iter }
    }
}

impl<I> Iterator for BitPacker<I> where I: Iterator<Item = bool> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0;

        for offset in 0..8 {
            // Try to get a bit from the input stream
            if let Some(bit) = self.iter.next() {
                if bit {
                    // Add the bit at the correct offset
                    result |= 1 << offset;
                }
            } else {
                // If there are no more bits, and this was the start of a byte
                // return None, otherwise return the partial result
                if offset == 0 {
                    return None
                } else {
                    return Some(result)
                }
            }
        }

        Some(result)
    }
}
