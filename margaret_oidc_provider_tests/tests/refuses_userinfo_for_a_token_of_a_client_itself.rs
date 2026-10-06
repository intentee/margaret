use std::time::Duration;

use chrono::Utc;
use uuid::Uuid;

use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oidc_provider::userinfo_authentication::UserinfoAuthentication;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;
use margaret_oidc_provider_tests::request_authorized_by::request_authorized_by;
use margaret_oidc_provider_tests::unserved_provider::UnservedProvider;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[test]
fn refuses_userinfo_for_a_token_of_a_client_itself() {
    let provider = UnservedProvider::create();
    let issuer = provider
        .issuance
        .token_issuance()
        .issuer
        .as_str()
        .to_string();
    let issued_at = NumericDate::from(Utc::now());
    let access_token = provider
        .roller
        .jwks_secret_holder()
        .get()
        .current()
        .sign_json(
            &ResourceAccessTokenClaims {
                client_id: "portal".parse().expect("the client identifier is visible"),
                scope: "openid"
                    .parse::<ScopeList>()
                    .expect("the scope is a scope list"),
                subject: "portal".to_string(),
            }
            .to_payload(&RegisteredClaims {
                aud: AudienceClaim::Single(issuer.clone()),
                exp: issued_at.after(Duration::from_mins(1)),
                iat: issued_at,
                iss: issuer,
                jti: Some(Uuid::new_v4().to_string()),
                nbf: None,
            }),
            JwtType::AccessToken,
        );
    let endpoint = UserinfoEndpoint::create(provider.secret_store, provider.issuance.as_ref());

    assert!(matches!(
        endpoint.authenticate(&request_authorized_by(&format!("Bearer {access_token}"))),
        UserinfoAuthentication::Refused(response) if response.status() == 401
    ));
}
