use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static COUNTS: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];

pub(crate) fn record(level: usize) {
    COUNTS[level].fetch_add(1, Ordering::Relaxed);
}

#[must_use]
pub fn construction_counts() -> [usize; 8] {
    std::array::from_fn(|position| COUNTS[position].load(Ordering::Relaxed))
}
