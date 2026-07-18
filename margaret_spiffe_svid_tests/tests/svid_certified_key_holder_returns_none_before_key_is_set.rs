use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;

#[test]
fn returns_none_before_key_is_set() {
    let holder = SvidCertifiedKeyHolder::default();

    assert!(holder.get().is_none());
}
