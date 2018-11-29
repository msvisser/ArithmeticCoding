pub struct BitUnpacker<I, D> {
    iter: I,
    data: Option<D>,
    offset: usize
}

impl<I> BitUnpacker<I, u8> where I: Iterator<Item = u8> {
    pub fn new(iter: I) -> Self {
        // Start the new bit unpacker at offset 8, so it will read in a new
        // byte on the first iteration
        Self {
            iter: iter,
            data: None,
            offset: 8
        }
    }
}

impl<I> Iterator for BitUnpacker<I, u8> where I: Iterator<Item = u8> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // When the offset is past the end of a byte, get the next data element
        if self.offset == 8 {
            self.data = self.iter.next();
            self.offset = 0;
        }

        if let Some(data) = self.data {
            // If there is a data byte, take the bit based on the offset
            let val = ((data >> self.offset) & 1) == 1;
            // and increment the offset for the next bit
            self.offset += 1;
            Some(val)
        } else {
            // Otherwise return None
            None
        }
    }
}
