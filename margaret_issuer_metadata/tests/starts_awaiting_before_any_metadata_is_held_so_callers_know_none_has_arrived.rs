use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;

#[test]
fn starts_awaiting_before_any_metadata_is_held_so_callers_know_none_has_arrived() {
    assert!(matches!(
        IssuerMetadata::awaiting().holding(),
        MetadataHolding::Awaiting
    ));
}
