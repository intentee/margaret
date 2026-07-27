use std::sync::atomic::AtomicUsize;

pub(crate) static COUNTS: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];
