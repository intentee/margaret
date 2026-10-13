use std::pin::Pin;

use tokio::time::Sleep;

pub(crate) enum StallWait {
    Idle,
    Waiting { stall: Pin<Box<Sleep>> },
}
