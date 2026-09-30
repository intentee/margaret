use chrono::Utc;
use serde::Deserialize;

use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_attribution::JwtAttribution;
use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::jwt_profiling::JwtProfiling;
use margaret_jwt_verification::presented_jwt::PresentedJwt;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::issuer_verification::IssuerVerification;

use crate::end_user_subject::END_USER_SUBJECT;
use crate::margaret_client::MargaretClient;
use crate::provider_fixture::ProviderFixture;
use crate::signed_in_portal::SignedInPortal;

#[derive(Deserialize)]
struct ResourceClaims {
    client_id: String,
    sub: String,
}

#[tokio::test]
async fn serves_an_access_token_a_margaret_resource_server_admits() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let client = MargaretClient::of_portal(&fixture).await;
    let SignedInPortal { signed_in, .. } = SignedInPortal::through(&fixture, &client).await;
    let trusted_issuer = &client.server.trusted_issuer;
    let JwtPresentation::Presented(presented) =
        PresentedJwt::present(signed_in.access_token.secret())
    else {
        panic!("the access token is a jws");
    };
    let JwtAttribution::Attributed(attributed) =
        presented.attribute_to(&trusted_issuer.trust.token_trust().issuer)
    else {
        panic!("the access token is issued by the provider");
    };
    let JwtProfiling::Profiled(profiled) = attributed.profile::<AccessTokenProfile>() else {
        panic!("the access token is an at+jwt");
    };
    let IssuerVerification::Verified(verified) = trusted_issuer
        .verify::<ResourceClaims, AccessTokenProfile>(
            &profiled,
            &trusted_issuer.trust.token_trust().audience,
            NumericDate::from(Utc::now()),
        )
        .await
    else {
        panic!("the resource server admits the access token");
    };

    assert_eq!(verified.claims.client_id, "portal");
    assert_eq!(verified.claims.sub, END_USER_SUBJECT.to_string());

    client.stop().await;
    fixture.stop().await;
}
