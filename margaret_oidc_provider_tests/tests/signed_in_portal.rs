use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::Arc;

use cookie::Cookie;
use http::HeaderValue;
use http::Method;
use http::header::COOKIE;
use url::Url;

use margaret_http::response::Response;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::sign_in_request::SignInRequest;
use margaret_oidc_sign_in::signed_in::SignedIn;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_validation::validate::validate;

use crate::authentication_claims::AuthenticationClaims;
use crate::margaret_client::MargaretClient;
use crate::portal_callback::PORTAL_CALLBACK;
use crate::provider_fixture::ProviderFixture;
use crate::signed_in_end_user::signed_in_end_user;

fn header<'response>(response: &'response Response, name: &str) -> &'response str {
    response
        .headers()
        .iter()
        .find(|header| header.name == name)
        .map(|header| header.value.as_str())
        .expect("the response carries the header")
}

pub struct SignedInPortal {
    pub flow: SignInFlow,
    pub signed_in: SignedIn<AuthenticationClaims>,
}

impl SignedInPortal {
    pub async fn through(fixture: &ProviderFixture, client: &MargaretClient) -> Self {
        let flow = SignInFlow::create(
            Arc::clone(&client.server),
            Arc::new(rolled_store(fresh_p256_secret())),
        );
        let SignInBeginning::Redirected(redirect) = flow
            .begin(SignInRequest {
                callback: Url::parse(PORTAL_CALLBACK).expect("the callback is a url"),
                scopes: BTreeSet::from([
                    "openid".parse().expect("the scope is a scope token"),
                    "profile".parse().expect("the scope is a scope token"),
                ]),
            })
            .await
            .expect("the transaction is signed")
        else {
            panic!("the sign-in redirects to the provider");
        };
        let parameters: HashMap<String, String> = Url::parse(header(&redirect, "location"))
            .expect("the authorization url is a url")
            .query_pairs()
            .into_owned()
            .collect();
        let transaction = Cookie::parse(header(&redirect, "set-cookie").to_string())
            .expect("the transaction cookie parses");
        let callback = crate::redirection::Redirection::of_authorization(
            &fixture
                .authorization
                .authorize(
                    validate(&parameters),
                    &EndUserAuthentication::Authenticated(signed_in_end_user()),
                )
                .await
                .expect("the authorization reaches its state"),
        );
        let mut request = FixtureRequest::new(
            Method::GET,
            &format!(
                "/callback?{}",
                callback
                    .location
                    .query()
                    .expect("the callback carries the code")
            ),
        );

        request.headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!("{}={}", transaction.name(), transaction.value()))
                .expect("the cookie is a header value"),
        );

        let signed_in = match flow
            .complete::<AuthenticationClaims>(&request.into_request())
            .await
            .expect("the sign-in completes")
        {
            SignInCompletion::SignedIn(signed_in) => signed_in,
            SignInCompletion::Refused(refusal) => {
                panic!("the provider refused the sign-in: {refusal}")
            }
            SignInCompletion::SigningKeysAwaited => {
                panic!("the signing keys of the provider are held")
            }
            SignInCompletion::Unavailable(unavailability) => {
                panic!("the provider is available: {unavailability}")
            }
        };

        Self { flow, signed_in }
    }
}
