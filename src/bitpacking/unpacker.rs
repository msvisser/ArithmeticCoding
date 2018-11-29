pub struct BitUnpacker<I, D> {
    iter: I,
    data: Option<D>,
    offset: usize
}

impl<I> BitUnpacker<I, u8> where I: Iterator<Item = u8> {
    pub fn new(iter: I) -> Self {
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
        if self.offset == 8 {
            self.data = self.iter.next();
            self.offset = 0;
        }

        if let Some(data) = self.data {
            let val = ((data >> self.offset) & 1) == 1;
            self.offset += 1;
            Some(val)
        } else {
            None
        }
    }
}
