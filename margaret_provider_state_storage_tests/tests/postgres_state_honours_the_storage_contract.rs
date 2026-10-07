use margaret_provider_state_storage_tests::postgres_state::PostgresState;
use margaret_provider_state_storage_tests::scenarios::spends_a_client_assertion_once::spends_a_client_assertion_once;
use margaret_provider_state_storage_tests::scenarios::spends_the_assertions_of_distinct_clients_independently::spends_the_assertions_of_distinct_clients_independently;
use margaret_provider_state_storage_tests::scenarios::spends_an_assertion_again_once_its_record_expired::spends_an_assertion_again_once_its_record_expired;
use margaret_provider_state_storage_tests::scenarios::reports_an_expired_client_assertion::reports_an_expired_client_assertion;
use margaret_provider_state_storage_tests::scenarios::spends_a_client_assertion_presented_concurrently_once::spends_a_client_assertion_presented_concurrently_once;
use margaret_provider_state_storage_tests::scenarios::approves_a_pending_authorization_into_a_code::approves_a_pending_authorization_into_a_code;
use margaret_provider_state_storage_tests::scenarios::denies_a_pending_authorization::denies_a_pending_authorization;
use margaret_provider_state_storage_tests::scenarios::forgets_a_pending_authorization_decided_by_another_subject::forgets_a_pending_authorization_decided_by_another_subject;
use margaret_provider_state_storage_tests::scenarios::opens_a_refresh_family_when_spending_a_code::opens_a_refresh_family_when_spending_a_code;
use margaret_provider_state_storage_tests::scenarios::refuses_to_revoke_a_refresh_token_of_another_client::refuses_to_revoke_a_refresh_token_of_another_client;
use margaret_provider_state_storage_tests::scenarios::reports_an_unknown_authorization_code::reports_an_unknown_authorization_code;
use margaret_provider_state_storage_tests::scenarios::reports_an_unknown_refresh_token::reports_an_unknown_refresh_token;
use margaret_provider_state_storage_tests::scenarios::reports_the_revocation_of_an_unknown_refresh_token::reports_the_revocation_of_an_unknown_refresh_token;
use margaret_provider_state_storage_tests::scenarios::reports_the_rotation_of_an_unknown_refresh_token::reports_the_rotation_of_an_unknown_refresh_token;
use margaret_provider_state_storage_tests::scenarios::reports_the_spending_of_an_unknown_code::reports_the_spending_of_an_unknown_code;
use margaret_provider_state_storage_tests::scenarios::revokes_the_family_of_a_code_spent_concurrently::revokes_the_family_of_a_code_spent_concurrently;
use margaret_provider_state_storage_tests::scenarios::revokes_the_family_of_a_refresh_token::revokes_the_family_of_a_refresh_token;
use margaret_provider_state_storage_tests::scenarios::revokes_the_family_of_a_refresh_token_rotated_concurrently::revokes_the_family_of_a_refresh_token_rotated_concurrently;
use margaret_provider_state_storage_tests::scenarios::revokes_the_family_of_a_replayed_refresh_token::revokes_the_family_of_a_replayed_refresh_token;
use margaret_provider_state_storage_tests::scenarios::revokes_the_refresh_family_of_a_replayed_code::revokes_the_refresh_family_of_a_replayed_code;
use margaret_provider_state_storage_tests::scenarios::rotates_a_current_refresh_token::rotates_a_current_refresh_token;
use margaret_provider_state_storage_tests::scenarios::spends_an_authorization_code_once::spends_an_authorization_code_once;

#[tokio::test]
async fn postgres_state_honours_the_storage_contract() {
    let postgres = PostgresState::with_tables().await;

    approves_a_pending_authorization_into_a_code(postgres.state.as_ref()).await;

    denies_a_pending_authorization(postgres.state.as_ref()).await;

    forgets_a_pending_authorization_decided_by_another_subject(postgres.state.as_ref()).await;

    opens_a_refresh_family_when_spending_a_code(postgres.state.as_ref()).await;

    refuses_to_revoke_a_refresh_token_of_another_client(postgres.state.as_ref()).await;

    reports_an_unknown_authorization_code(postgres.state.as_ref()).await;

    reports_an_unknown_refresh_token(postgres.state.as_ref()).await;

    reports_the_revocation_of_an_unknown_refresh_token(postgres.state.as_ref()).await;

    reports_the_rotation_of_an_unknown_refresh_token(postgres.state.as_ref()).await;

    reports_the_spending_of_an_unknown_code(postgres.state.as_ref()).await;

    revokes_the_family_of_a_code_spent_concurrently(postgres.state.as_ref()).await;

    revokes_the_family_of_a_refresh_token(postgres.state.as_ref()).await;

    revokes_the_family_of_a_refresh_token_rotated_concurrently(postgres.state.as_ref()).await;

    revokes_the_family_of_a_replayed_refresh_token(postgres.state.as_ref()).await;

    revokes_the_refresh_family_of_a_replayed_code(postgres.state.as_ref()).await;

    rotates_a_current_refresh_token(postgres.state.as_ref()).await;

    spends_an_authorization_code_once(postgres.state.as_ref()).await;

    spends_a_client_assertion_once(postgres.state.as_ref()).await;

    spends_the_assertions_of_distinct_clients_independently(postgres.state.as_ref()).await;

    spends_an_assertion_again_once_its_record_expired(postgres.state.as_ref()).await;

    reports_an_expired_client_assertion(postgres.state.as_ref()).await;

    spends_a_client_assertion_presented_concurrently_once(postgres.state.as_ref()).await;
}
