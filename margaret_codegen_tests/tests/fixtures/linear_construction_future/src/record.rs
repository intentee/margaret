use std::sync::atomic::Ordering;

use crate::counts::COUNTS;

pub(crate) fn record(level: usize) {
    COUNTS[level].fetch_add(1, Ordering::Relaxed);
}
