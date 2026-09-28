use std::array;
use std::sync::atomic::Ordering;

use crate::counts::COUNTS;

#[must_use]
pub fn construction_counts() -> [usize; 8] {
    array::from_fn(|position| COUNTS[position].load(Ordering::Relaxed))
}
