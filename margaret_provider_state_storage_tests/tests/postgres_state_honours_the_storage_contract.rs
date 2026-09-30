use margaret_provider_state_storage_tests::postgres_state::PostgresState;
use margaret_provider_state_storage_tests::scenarios::approves_a_pending_authorization_into_a_code::approves_a_pending_authorization_into_a_code;
use margaret_provider_state_storage_tests::scenarios::burns_a_code_presented_by_another_client::burns_a_code_presented_by_another_client;
use margaret_provider_state_storage_tests::scenarios::burns_a_code_presented_with_another_callback::burns_a_code_presented_with_another_callback;
use margaret_provider_state_storage_tests::scenarios::burns_a_code_presented_with_another_challenge::burns_a_code_presented_with_another_challenge;
use margaret_provider_state_storage_tests::scenarios::denies_a_pending_authorization::denies_a_pending_authorization;
use margaret_provider_state_storage_tests::scenarios::forgets_a_pending_authorization_decided_by_another_subject::forgets_a_pending_authorization_decided_by_another_subject;
use margaret_provider_state_storage_tests::scenarios::opens_a_refresh_family_when_redeeming_a_code::opens_a_refresh_family_when_redeeming_a_code;
use margaret_provider_state_storage_tests::scenarios::redeems_an_authorization_code_once::redeems_an_authorization_code_once;
use margaret_provider_state_storage_tests::scenarios::refuses_to_revoke_a_refresh_token_of_another_client::refuses_to_revoke_a_refresh_token_of_another_client;
use margaret_provider_state_storage_tests::scenarios::refuses_to_rotate_a_token_beyond_its_granted_scope::refuses_to_rotate_a_token_beyond_its_granted_scope;
use margaret_provider_state_storage_tests::scenarios::refuses_to_rotate_a_token_for_another_client::refuses_to_rotate_a_token_for_another_client;
use margaret_provider_state_storage_tests::scenarios::reports_an_unknown_authorization_code::reports_an_unknown_authorization_code;
use margaret_provider_state_storage_tests::scenarios::reports_an_unknown_refresh_token::reports_an_unknown_refresh_token;
use margaret_provider_state_storage_tests::scenarios::reports_the_revocation_of_an_unknown_refresh_token::reports_the_revocation_of_an_unknown_refresh_token;
use margaret_provider_state_storage_tests::scenarios::revokes_the_family_of_a_refresh_token::revokes_the_family_of_a_refresh_token;
use margaret_provider_state_storage_tests::scenarios::revokes_the_family_of_a_replayed_refresh_token::revokes_the_family_of_a_replayed_refresh_token;
use margaret_provider_state_storage_tests::scenarios::revokes_the_refresh_family_of_a_replayed_code::revokes_the_refresh_family_of_a_replayed_code;
use margaret_provider_state_storage_tests::scenarios::rotates_a_current_refresh_token::rotates_a_current_refresh_token;
use margaret_provider_state_storage_tests::scenarios::rotates_a_token_narrowed_within_its_granted_scope::rotates_a_token_narrowed_within_its_granted_scope;

#[tokio::test]
async fn postgres_state_honours_the_storage_contract() {
    let postgres = PostgresState::with_tables().await;

    approves_a_pending_authorization_into_a_code(&postgres.state).await;

    burns_a_code_presented_by_another_client(&postgres.state).await;

    burns_a_code_presented_with_another_callback(&postgres.state).await;

    burns_a_code_presented_with_another_challenge(&postgres.state).await;

    denies_a_pending_authorization(&postgres.state).await;

    forgets_a_pending_authorization_decided_by_another_subject(&postgres.state).await;

    opens_a_refresh_family_when_redeeming_a_code(&postgres.state).await;

    redeems_an_authorization_code_once(&postgres.state).await;

    refuses_to_revoke_a_refresh_token_of_another_client(&postgres.state).await;

    refuses_to_rotate_a_token_beyond_its_granted_scope(&postgres.state).await;

    refuses_to_rotate_a_token_for_another_client(&postgres.state).await;

    reports_an_unknown_authorization_code(&postgres.state).await;

    reports_an_unknown_refresh_token(&postgres.state).await;

    reports_the_revocation_of_an_unknown_refresh_token(&postgres.state).await;

    revokes_the_family_of_a_refresh_token(&postgres.state).await;

    revokes_the_family_of_a_replayed_refresh_token(&postgres.state).await;

    revokes_the_refresh_family_of_a_replayed_code(&postgres.state).await;

    rotates_a_current_refresh_token(&postgres.state).await;

    rotates_a_token_narrowed_within_its_granted_scope(&postgres.state).await;
}
