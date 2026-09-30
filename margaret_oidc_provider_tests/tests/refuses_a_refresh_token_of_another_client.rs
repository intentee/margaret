use std::collections::BTreeSet;

use serde_json::json;

use margaret_accepted_clients::grant_type::GrantType;

use crate::client_credentials::ClientCredentials;
use crate::portal_client::portal_client;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;
use crate::refreshed_tokens::refreshed_tokens;
use crate::service_client::service_client;
use crate::service_secret::SERVICE_SECRET;

#[tokio::test]
async fn refuses_a_refresh_token_of_another_client() {
    let mut service = service_client();

    service.grants = BTreeSet::from([GrantType::RefreshToken]);

    let fixture = ProviderFixture::serving(vec![portal_client(), service], Vec::new()).await;
    let refresh_token = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let foreign = fixture
        .post_form(
            "/token",
            &ClientCredentials::Basic {
                client_id: "service",
                secret: SERVICE_SECRET,
            },
            &json!({"grant_type": "refresh_token", "refresh_token": refresh_token}),
        )
        .await;

    assert_eq!(foreign.status, 400);
    assert_eq!(foreign.body["error"], "invalid_grant");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
