use std::collections::BTreeSet;

use url::Url;

use margaret_oidc_sign_in::sign_in_request::SignInRequest;

use crate::portal_callback::PORTAL_CALLBACK;

/// # Panics
///
/// Panics when the portal callback or its scopes are malformed.
#[must_use]
pub fn portal_sign_in_request() -> SignInRequest {
    SignInRequest {
        callback: Url::parse(PORTAL_CALLBACK).expect("the callback is a url"),
        scopes: BTreeSet::from([
            "openid".parse().expect("the scope is a scope token"),
            "profile".parse().expect("the scope is a scope token"),
        ]),
    }
}
