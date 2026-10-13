use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;

#[test]
fn returns_none_before_a_key_is_set_so_callers_never_see_a_placeholder_key() {
    let holder = SvidCertifiedKeyHolder::default();

    assert!(holder.get().is_none());
}
