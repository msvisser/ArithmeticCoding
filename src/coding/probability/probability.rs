pub trait Probability {
    fn cumulative_bottom(&self, symbol: usize) -> u64;
    fn cumulative_top(&self, symbol: usize) -> u64;
    fn total(&self) -> u64;
    fn increment(&mut self, symbol: usize);
    fn update_last(&mut self, symbol: usize);
}
