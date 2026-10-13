use margaret::framework::macros::scheduled_with_tick_timer;

#[scheduled_with_tick_timer(interval = crate::P)]
enum Bad {}
