use std::collections::BTreeMap;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http_tests::redirection::Redirection;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;

#[must_use]
pub fn sign_in_callback(begun: &BegunSignIn, callback: &Response) -> Request {
    let redirection = Redirection::of(callback);

    callback_request(
        &begun.cookie_pair(),
        &redirection
            .parameters
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
            .collect::<BTreeMap<&str, &str>>(),
    )
}
