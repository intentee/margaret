use std::time::Duration;

use uuid::Uuid;

pub const ARTICLE_BATCH: usize = 2_000;

pub const CI_PUBLISHER: Uuid = Uuid::from_u128(901);

pub const FEATURED_ARTICLE_ID: Uuid = Uuid::from_u128(100);

pub const SWEEP_INTERVAL: Duration = Duration::from_secs(1);
