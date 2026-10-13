use std::time::Duration;

use uuid::Uuid;

pub const ALICE: Uuid = Uuid::from_u128(1);

pub const NOTE_SWEEP_INTERVAL: Duration = Duration::from_secs(1);

pub const STREAM_BATCH: usize = 2_000;
