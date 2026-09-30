use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;

#[test]
fn awaits_the_first_metadata() {
    assert!(matches!(
        IssuerMetadata::awaiting().holding(),
        MetadataHolding::Awaiting
    ));
}
