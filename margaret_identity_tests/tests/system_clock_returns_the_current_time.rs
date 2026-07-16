use chrono::DateTime;

use margaret_identity::clock::Clock;
use margaret_identity::system_clock::SystemClock;

#[test]
fn system_clock_returns_the_current_time() {
    let clock = SystemClock::create();
    let threshold = DateTime::from_timestamp(1_700_000_000, 0).expect("a valid timestamp");

    assert!(clock.now() > threshold);
}
