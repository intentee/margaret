use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;

#[test]
fn starts_awaiting_the_first_key_set_rather_than_holding_an_empty_one() {
    assert!(matches!(
        IssuerKeySet::awaiting().snapshot().holding,
        KeySetHolding::Awaiting
    ));
}
