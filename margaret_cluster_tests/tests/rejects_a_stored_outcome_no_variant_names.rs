use margaret::framework::active_record::enum_column::EnumColumn;
use margaret_cluster_fixture::models::token_acquisition_outcome::TokenAcquisitionOutcome;

#[test]
fn rejects_a_stored_outcome_no_variant_names() {
    assert!(TokenAcquisitionOutcome::from_variant_name("Pending").is_none());
}
