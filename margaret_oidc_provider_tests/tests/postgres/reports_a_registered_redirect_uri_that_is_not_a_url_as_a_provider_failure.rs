use std::sync::Arc;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_oidc_provider::authorization_endpoint::AuthorizationEndpoint;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::constants::SPA_CLIENT;
use margaret_oidc_provider_tests::constants::SPA_CODE_GRANT;
use margaret_oidc_provider_tests::provider_issuance::provider_issuance;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;
use margaret_oidc_provider_tests::started_with_provider_state::started_with_provider_state;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

const UNPARSEABLE_REDIRECT_URI: &str = "spa callback";

#[tokio::test]
async fn reports_a_registered_redirect_uri_that_is_not_a_url_as_a_provider_failure() {
    let started = started_with_provider_state().await;
    let authorization = AuthorizationEndpoint::create(
        Arc::new(AcceptedClients::create(
            vec![Arc::new(RegisteredClient::public_with_code_grant(
                SPA_CLIENT,
                Arc::clone(&started.database),
                SPA_CODE_GRANT,
                vec![UNPARSEABLE_REDIRECT_URI.to_string()],
            ))],
            provider_issuance(),
        )),
        "https://localhost/authorize",
        provider_issuance(),
    );

    assert!(matches!(
        authorization
            .authorize(
                validated_form(&with_parameter(
                    spa_parameters(),
                    "redirect_uri",
                    UNPARSEABLE_REDIRECT_URI,
                )),
                &EndUserAuthentication::Anonymous,
            )
            .await,
        Err(ProviderError::RegisteredRedirectUri { client_id, redirect_uri, .. })
            if client_id == "spa" && redirect_uri == UNPARSEABLE_REDIRECT_URI
    ));
}
