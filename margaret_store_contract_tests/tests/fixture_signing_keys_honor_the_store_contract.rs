use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

use margaret_store_contract_tests::concurrent_signing_keys_creations_admit_one::concurrent_signing_keys_creations_admit_one;
use margaret_store_contract_tests::concurrent_signing_keys_replacements_admit_one::concurrent_signing_keys_replacements_admit_one;
use margaret_store_contract_tests::signing_keys_creation_refuses_existing_keys::signing_keys_creation_refuses_existing_keys;
use margaret_store_contract_tests::signing_keys_replacement_refuses_absent_keys::signing_keys_replacement_refuses_absent_keys;
use margaret_store_contract_tests::signing_keys_replacement_refuses_another_generation::signing_keys_replacement_refuses_another_generation;
use margaret_store_contract_tests::signing_keys_start_absent::signing_keys_start_absent;

#[tokio::test]
async fn signing_keys_start_absent_for_the_fixture_store() {
    signing_keys_start_absent(&FixtureSigningKeys::empty()).await;
}

#[tokio::test]
async fn concurrent_signing_keys_creations_admit_one_for_the_fixture_store() {
    concurrent_signing_keys_creations_admit_one(&FixtureSigningKeys::empty()).await;
}

#[tokio::test]
async fn signing_keys_creation_refuses_existing_keys_for_the_fixture_store() {
    signing_keys_creation_refuses_existing_keys(&FixtureSigningKeys::empty()).await;
}

#[tokio::test]
async fn concurrent_signing_keys_replacements_admit_one_for_the_fixture_store() {
    concurrent_signing_keys_replacements_admit_one(&FixtureSigningKeys::empty()).await;
}

#[tokio::test]
async fn signing_keys_replacement_refuses_another_generation_for_the_fixture_store() {
    signing_keys_replacement_refuses_another_generation(&FixtureSigningKeys::empty()).await;
}

#[tokio::test]
async fn signing_keys_replacement_refuses_absent_keys_for_the_fixture_store() {
    signing_keys_replacement_refuses_absent_keys(&FixtureSigningKeys::empty()).await;
}
