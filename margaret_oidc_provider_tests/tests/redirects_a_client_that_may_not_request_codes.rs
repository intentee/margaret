use std::collections::BTreeSet;

use margaret_accepted_clients::grant_type::GrantType;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_client::portal_client;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;

#[tokio::test]
async fn redirects_a_client_that_may_not_request_codes() {
    let mut portal = portal_client();

    portal.grants = BTreeSet::from([GrantType::ClientCredentials]);

    let fixture = ProviderFixture::serving(vec![portal], Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&portal_parameters()),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    assert_eq!(
        Redirection::of_authorization(&outcome).parameter("error"),
        "unauthorized_client"
    );

    fixture.stop().await;
}
