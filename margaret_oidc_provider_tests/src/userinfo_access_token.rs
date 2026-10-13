use std::collections::BTreeSet;
use std::time::Duration;

use chrono::Utc;
use uuid::Uuid;

use margaret_identity_session::issued_access_token_claims::IssuedAccessTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::unserved_provider::UnservedProvider;

#[must_use]
pub fn userinfo_access_token(provider: &UnservedProvider, subject: &str) -> String {
    let issuer = provider.issuance.issuer.to_string();
    let issued_at = NumericDate::from(Utc::now());

    provider.secrets.get().current().sign_json(
        &IssuedAccessTokenClaims {
            client_id: "portal",
            scopes: BTreeSet::from(["openid".to_string()]),
            subject: subject.to_string(),
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
    )
}
