use std::time::Duration;

use trzcina::TickContext;

#[must_use]
pub fn first_tick_context() -> TickContext {
    TickContext {
        elapsed_since_start: Duration::ZERO,
        ticks_since_start: 0,
        since_last_tick: Duration::ZERO,
    }
}
