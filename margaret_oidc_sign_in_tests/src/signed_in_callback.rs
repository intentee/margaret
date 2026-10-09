use std::collections::BTreeMap;

use margaret_http::request::Request;
use margaret_jose_parameters::jwt_type::JwtType;

use crate::begun_sign_in::BegunSignIn;
use crate::callback_request::callback_request;
use crate::id_token_claims::id_token_claims;
use crate::issued_token_answer::issued_token_answer;
use crate::sign_in_fixture::SignInFixture;

/// # Panics
///
/// Panics when the token endpoint already answers.
#[must_use]
pub fn signed_in_callback(fixture: &SignInFixture, begun: &BegunSignIn) -> Request {
    assert!(
        fixture
            .token_endpoint
            .answer
            .set(issued_token_answer(
                &fixture.issuer_secret.current().sign_json(
                    &id_token_claims(begun.authorization_parameter("nonce")),
                    JwtType::Jwt
                )
            ))
            .is_ok()
    );

    callback_request(
        &begun.cookie_pair(),
        &BTreeMap::from([
            ("code", "SplxlOBeZQQYbYS6WxSbIA"),
            ("iss", "https://localhost"),
            ("state", begun.authorization_parameter("state")),
        ]),
    )
}
