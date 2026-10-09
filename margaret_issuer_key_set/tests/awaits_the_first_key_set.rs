use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;

#[test]
fn awaits_the_first_key_set() {
    assert!(matches!(
        IssuerKeySet::awaiting().snapshot().holding,
        KeySetHolding::Awaiting
    ));
}
