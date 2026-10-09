use std::collections::BTreeSet;

use url::Url;
use uuid::Uuid;

use margaret_authorization_grants::authorization_grant::AuthorizationGrant;
use margaret_oauth_vocabulary::code_challenge::CodeChallenge;
use margaret_oauth_vocabulary::code_verifier::CodeVerifier;
use margaret_oauth_vocabulary::scope::Scope;

use crate::contract_moment::contract_moment;

/// # Panics
///
/// Panics when the contract callback is not a url.
#[must_use]
pub fn contract_grant() -> AuthorizationGrant {
    AuthorizationGrant {
        auth_time: contract_moment(),
        client_id: "contract-client".to_string(),
        code_challenge: CodeChallenge::of(&CodeVerifier::generate()),
        nonce: Some("contract-nonce".to_string()),
        redirect_uri: Url::parse("https://client.example/callback")
            .expect("the contract callback is a url"),
        scopes: BTreeSet::from([Scope::openid()]),
        subject: Uuid::new_v4(),
    }
}
