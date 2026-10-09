use std::collections::BTreeSet;
use std::time::Duration;

use chrono::Utc;
use uuid::Uuid;

use margaret_identity_session::issued_access_token_claims::IssuedAccessTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_oidc_provider::userinfo_authentication::UserinfoAuthentication;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_oidc_provider_tests::request_authorized_by::request_authorized_by;
use margaret_oidc_provider_tests::unserved_provider::UnservedProvider;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[tokio::test]
async fn refuses_userinfo_for_a_token_of_a_client_itself() {
    let provider = UnservedProvider::create();
    let issuer = provider.issuance.issuer.to_string();
    let issued_at = NumericDate::from(Utc::now());
    let access_token = provider.secrets.get().current().sign_json(
        &IssuedAccessTokenClaims {
            client_id: "portal",
            scopes: BTreeSet::from(["openid".to_string()]),
            subject: "portal".to_string(),
        }
        .to_payload(&RegisteredClaims {
            aud: AudienceClaim::Single(issuer.clone()),
            exp: issued_at.after(Duration::from_mins(1)),
            iat: Some(issued_at),
            iss: issuer,
            jti: Some(Uuid::new_v4().to_string()),
            nbf: None,
        }),
        JwtType::AccessToken,
    );
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance);

    assert!(matches!(
        endpoint.authenticate(&request_authorized_by(&format!("Bearer {access_token}"))),
        UserinfoAuthentication::Refused(response) if response.status() == 401
    ));
}
