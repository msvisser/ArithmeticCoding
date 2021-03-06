const PRECISION: usize = 32;
const WHOLE: u64 = 1 << PRECISION;
const HALF: u64 = WHOLE / 2;
const QUARTER: u64 = WHOLE / 4;

mod encode;
pub use self::encode::ACEncoder;

mod decode;
pub use self::decode::ACDecoder;

mod probability;
pub use self::probability::{Probability, ProbabilityZeroOrder, ProbabilityFirstOrder};

mod range;
pub use self::range::Range;
