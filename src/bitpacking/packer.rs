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
            if let Some(bit) = self.iter.next() {
                if bit {
                    result |= 1 << offset;
                }
            } else {
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
