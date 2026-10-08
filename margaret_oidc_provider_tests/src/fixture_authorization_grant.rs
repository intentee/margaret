use std::collections::BTreeSet;

use chrono::Utc;
use url::Url;

use margaret_authorization_grants::authorization_grant::AuthorizationGrant;
use margaret_oauth_vocabulary::code_challenge::CodeChallenge;
use margaret_oauth_vocabulary::scope::Scope;

use crate::pkce_verifier::PKCE_VERIFIER;
use crate::signed_in_end_user::signed_in_end_user;

/// # Panics
///
/// Panics when the fixture verifier, scope or callback is rejected.
#[must_use]
pub fn fixture_authorization_grant(client_id: &str, redirect_uri: &str) -> AuthorizationGrant {
    AuthorizationGrant {
        auth_time: Utc::now(),
        client_id: client_id.to_string(),
        code_challenge: CodeChallenge::of(
            &serde_json::from_value(serde_json::Value::String(PKCE_VERIFIER.to_string()))
                .expect("the fixture verifier is accepted"),
        ),
        nonce: None,
        redirect_uri: Url::parse(redirect_uri).expect("the fixture callback is a url"),
        scopes: BTreeSet::from([Scope::openid()]),
        subject: signed_in_end_user().subject,
    }
}
