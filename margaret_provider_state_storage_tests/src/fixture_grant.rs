use std::collections::BTreeSet;

use chrono::DateTime;
use url::Url;
use uuid::Uuid;

use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;

use crate::fixture_client_id::fixture_client_id;

/// # Panics
///
/// Panics when the fixture scope, callback or instant is rejected.
#[must_use]
pub fn fixture_grant() -> AuthorizationGrant {
    AuthorizationGrant {
        auth_time: DateTime::from_timestamp(1_700_000_000, 0).expect("the instant is valid"),
        client_id: fixture_client_id(),
        code_challenge: "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM".to_string(),
        nonce: Some("nonce".to_string()),
        redirect_uri: Url::parse("https://client.example/callback").expect("the callback is a url"),
        scopes: BTreeSet::from(["openid".parse().expect("the scope is a scope token")]),
        subject: Uuid::from_u128(1),
    }
}
