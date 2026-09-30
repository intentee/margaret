use serde::Deserialize;

use margaret_oidc_sign_in::userinfo_fetch::UserinfoFetch;

use crate::end_user_subject::END_USER_SUBJECT;
use crate::margaret_client::MargaretClient;
use crate::provider_fixture::ProviderFixture;
use crate::signed_in_portal::SignedInPortal;

#[derive(Deserialize)]
struct NameClaims {
    name: String,
}

#[tokio::test]
async fn signs_in_a_margaret_client_through_the_provider() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let client = MargaretClient::of_portal(&fixture).await;
    let SignedInPortal { flow, signed_in } = SignedInPortal::through(&fixture, &client).await;
    let UserinfoFetch::Fetched(NameClaims { name }) =
        flow.userinfo::<NameClaims, _>(&signed_in).await
    else {
        panic!("the provider answers userinfo of the signed-in subject");
    };

    assert_eq!(signed_in.subject, END_USER_SUBJECT.to_string());
    assert!(signed_in.claims.auth_time > 0);
    assert_eq!(name, "Ada");

    client.stop().await;
    fixture.stop().await;
}
