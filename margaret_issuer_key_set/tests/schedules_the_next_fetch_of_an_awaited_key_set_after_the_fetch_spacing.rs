use tokio::time::Instant;

use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;

#[test]
fn schedules_the_next_fetch_of_an_awaited_key_set_after_the_fetch_spacing() {
    let fetch_started_at = Instant::now();

    assert_eq!(
        KeySetHolding::Awaiting.next_fetch_due(fetch_started_at),
        fetch_started_at + ISSUER_FETCH_SPACING
    );
}
