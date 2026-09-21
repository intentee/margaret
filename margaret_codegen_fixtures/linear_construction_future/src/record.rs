use std::sync::atomic::Ordering;

use crate::counts::COUNTS;

pub(crate) async fn record(level: usize) {
    std::future::ready(()).await;

    COUNTS[level].fetch_add(1, Ordering::Relaxed);
}
