use margaret_store_contract_tests::holding_a_pending_authorization_sweeps_expired_ones::holding_a_pending_authorization_sweeps_expired_ones;
use margaret_store_contract_tests::issuing_a_code_sweeps_expired_codes::issuing_a_code_sweeps_expired_codes;
use margaret_store_contract_tests::issuing_a_code_sweeps_expired_redeemed_codes::issuing_a_code_sweeps_expired_redeemed_codes;
use margaret_store_contract_tests::opening_a_refresh_family_sweeps_expired_families::opening_a_refresh_family_sweeps_expired_families;
use margaret_store_contract_tests::rotating_a_refresh_token_sweeps_expired_families::rotating_a_refresh_token_sweeps_expired_families;
use margaret_store_contract_tests::revoking_an_unopened_family_bars_it_for_the_refresh_family_lifetime::revoking_an_unopened_family_bars_it_for_the_refresh_family_lifetime;
use margaret_store_contract_tests::opening_a_refresh_family_sweeps_expired_revocations::opening_a_refresh_family_sweeps_expired_revocations;
use margaret_oidc_provider_tests::fixture_authorization_grants::FixtureAuthorizationGrants;

use margaret_store_contract_tests::code_redemption_finds_no_unknown_code::code_redemption_finds_no_unknown_code;
use margaret_store_contract_tests::concurrent_code_redemptions_admit_one::concurrent_code_redemptions_admit_one;
use margaret_store_contract_tests::concurrent_pending_authorization_takes_admit_one::concurrent_pending_authorization_takes_admit_one;
use margaret_store_contract_tests::concurrent_refresh_rotations_admit_one::concurrent_refresh_rotations_admit_one;
use margaret_store_contract_tests::opened_refresh_family_resolves_its_first_token::opened_refresh_family_resolves_its_first_token;
use margaret_store_contract_tests::pending_authorization_take_finds_no_unknown_authorization::pending_authorization_take_finds_no_unknown_authorization;
use margaret_store_contract_tests::refresh_family_revoked_before_opening_never_opens::refresh_family_revoked_before_opening_never_opens;
use margaret_store_contract_tests::refresh_rotation_finds_no_unknown_token::refresh_rotation_finds_no_unknown_token;
use margaret_store_contract_tests::refresh_token_lookup_finds_no_unknown_token::refresh_token_lookup_finds_no_unknown_token;
use margaret_store_contract_tests::revoked_refresh_family_neither_resolves_nor_rotates::revoked_refresh_family_neither_resolves_nor_rotates;

#[tokio::test]
async fn concurrent_pending_authorization_takes_admit_one_for_the_fixture_store() {
    concurrent_pending_authorization_takes_admit_one(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn pending_authorization_take_finds_no_unknown_authorization_for_the_fixture_store() {
    pending_authorization_take_finds_no_unknown_authorization(
        &FixtureAuthorizationGrants::undisturbed(),
    )
    .await;
}

#[tokio::test]
async fn concurrent_code_redemptions_admit_one_for_the_fixture_store() {
    concurrent_code_redemptions_admit_one(&FixtureAuthorizationGrants::undisturbed()).await;
}

#[tokio::test]
async fn code_redemption_finds_no_unknown_code_for_the_fixture_store() {
    code_redemption_finds_no_unknown_code(&FixtureAuthorizationGrants::undisturbed()).await;
}

#[tokio::test]
async fn opened_refresh_family_resolves_its_first_token_for_the_fixture_store() {
    opened_refresh_family_resolves_its_first_token(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn concurrent_refresh_rotations_admit_one_for_the_fixture_store() {
    concurrent_refresh_rotations_admit_one(&FixtureAuthorizationGrants::undisturbed()).await;
}

#[tokio::test]
async fn refresh_token_lookup_finds_no_unknown_token_for_the_fixture_store() {
    refresh_token_lookup_finds_no_unknown_token(&FixtureAuthorizationGrants::undisturbed()).await;
}

#[tokio::test]
async fn refresh_rotation_finds_no_unknown_token_for_the_fixture_store() {
    refresh_rotation_finds_no_unknown_token(&FixtureAuthorizationGrants::undisturbed()).await;
}

#[tokio::test]
async fn revoked_refresh_family_neither_resolves_nor_rotates_for_the_fixture_store() {
    revoked_refresh_family_neither_resolves_nor_rotates(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn refresh_family_revoked_before_opening_never_opens_for_the_fixture_store() {
    refresh_family_revoked_before_opening_never_opens(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn holding_a_pending_authorization_sweeps_expired_ones_for_the_fixture_store() {
    holding_a_pending_authorization_sweeps_expired_ones(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn issuing_a_code_sweeps_expired_codes_for_the_fixture_store() {
    issuing_a_code_sweeps_expired_codes(&FixtureAuthorizationGrants::undisturbed()).await;
}

#[tokio::test]
async fn opening_a_refresh_family_sweeps_expired_families_for_the_fixture_store() {
    opening_a_refresh_family_sweeps_expired_families(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn rotating_a_refresh_token_sweeps_expired_families_for_the_fixture_store() {
    rotating_a_refresh_token_sweeps_expired_families(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn revoking_an_unopened_family_bars_it_for_the_refresh_family_lifetime_for_the_fixture_store()
{
    revoking_an_unopened_family_bars_it_for_the_refresh_family_lifetime(
        &FixtureAuthorizationGrants::undisturbed(),
    )
    .await;
}

#[tokio::test]
async fn opening_a_refresh_family_sweeps_expired_revocations_for_the_fixture_store() {
    opening_a_refresh_family_sweeps_expired_revocations(&FixtureAuthorizationGrants::undisturbed())
        .await;
}

#[tokio::test]
async fn issuing_a_code_sweeps_expired_redeemed_codes_for_the_fixture_store() {
    issuing_a_code_sweeps_expired_redeemed_codes(&FixtureAuthorizationGrants::undisturbed()).await;
}
