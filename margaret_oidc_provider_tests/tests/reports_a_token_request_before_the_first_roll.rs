use std::collections::HashMap;
use std::sync::Arc;

use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider::token_endpoint::TokenEndpoint;
use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_validation::validate::validate;

use crate::request_authorized_by::request_authorized_by;
use crate::unrolled_provider::UnrolledProvider;

#[tokio::test]
async fn reports_a_token_request_before_the_first_roll() {
    let provider = UnrolledProvider::create();
    let endpoint = TokenEndpoint::create(
        provider.clients,
        Arc::new(MemoryProviderState::create()),
        Arc::new(SubjectTokenExchangers::create(Vec::new())),
        provider.secret_store,
        provider.issuance,
    );

    assert!(matches!(
        endpoint
            .respond(
                &request_authorized_by("Basic cG9ydGFsOnBvcnRhbC1zZWNyZXQ="),
                validate(&HashMap::from([(
                    "grant_type".to_string(),
                    "client_credentials".to_string()
                )])),
            )
            .await,
        Err(ProviderError::Signing(_))
    ));
}
