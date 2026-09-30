use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection::introspection_admission::IntrospectionAdmission;

use crate::end_user_subject::END_USER_SUBJECT;
use crate::margaret_client::MargaretClient;
use crate::provider_fixture::ProviderFixture;
use crate::request_authorized_by::request_authorized_by;
use crate::signed_in_portal::SignedInPortal;

#[tokio::test]
async fn introspects_an_access_token_for_a_margaret_resource_server() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let client = MargaretClient::of_portal(&fixture).await;
    let SignedInPortal { signed_in, .. } = SignedInPortal::through(&fixture, &client).await;
    let request = request_authorized_by(&format!("Bearer {}", signed_in.access_token.secret()));
    let IntrospectionAdmission::Admitted(introspected) =
        introspect_bearer_token::<serde_json::Value>(
            request.inputs.server.authorization(),
            &client.server,
        )
        .await
        .expect("the introspection is authenticated")
    else {
        panic!("the provider reports the access token as active");
    };

    assert_eq!(
        introspected.subject.as_deref(),
        Some(END_USER_SUBJECT.to_string().as_str())
    );

    client.stop().await;
    fixture.stop().await;
}
