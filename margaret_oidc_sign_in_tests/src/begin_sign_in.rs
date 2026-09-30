use std::collections::BTreeSet;

use url::Url;

use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::sign_in_request::SignInRequest;

/// # Panics
///
/// Panics when the sign-in transaction cannot be signed.
pub async fn begin_sign_in(flow: &SignInFlow) -> SignInBeginning {
    flow.begin(SignInRequest {
        callback: Url::parse("https://client.example/callback").expect("the callback is a url"),
        scopes: BTreeSet::from(["profile".parse().expect("the scope is a scope token")]),
    })
    .await
    .expect("the transaction is signed")
}
