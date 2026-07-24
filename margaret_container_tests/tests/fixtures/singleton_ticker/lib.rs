use std::sync::Arc;

#[singleton]
#[scheduled_with_tick_timer(interval = crate::schedule::PERIOD)]
struct Roller;

impl Roller {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Reader;

impl Reader {
    #[constructor]
    fn new(roller: Arc<Roller>) -> Self {}
}
