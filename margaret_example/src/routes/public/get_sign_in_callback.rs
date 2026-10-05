use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::http::redirect::Redirect;
use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret::framework::oidc_sign_in::signed_in::SignedIn;

use crate::auth::sign_in_claims::SignInClaims;
use crate::forms::session_cookie::SessionCookie;
use crate::margaret::oauth_clients::blog::SignInFlow;
use crate::margaret::routes::Routes;
use crate::stores::user_store::UserStore;

#[singleton]
#[responds_to_http(
    method = "get",
    name = "get_sign_in_callback",
    path = "/sign-in/callback",
    server = "public"
)]
pub struct GetSignInCallback {
    sign_in_flow: Arc<SignInFlow>,
    users: Arc<UserStore>,
}

impl GetSignInCallback {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(sign_in_flow: Arc<SignInFlow>, users: Arc<UserStore>) -> anyhow::Result<Self> {
        Ok(Self {
            sign_in_flow,
            users,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(&self, request: &Request, routes: &Routes) -> anyhow::Result<Response> {
        Ok(
            match self.sign_in_flow.complete::<SignInClaims>(request).await {
                SignInCompletion::SignedIn(SignedIn {
                    subject,
                    transaction_removal,
                    ..
                }) => self
                    .signed_in(&subject, routes)
                    .set_cookie(&transaction_removal),
                SignInCompletion::Refused(refusal) => Response::text(403, refusal.to_string()),
                SignInCompletion::SigningKeysAwaited => Response::text(
                    503,
                    "The signing keys of the identity server are not available yet",
                ),
                SignInCompletion::Unavailable(unavailability) => {
                    Response::text(503, unavailability.to_string())
                }
            },
        )
    }

    fn signed_in(&self, subject: &str, routes: &Routes) -> Response {
        match Uuid::parse_str(subject) {
            Ok(user_id) => match self.users.start_session(user_id) {
                Some(session) => Redirect::see_other(routes.public.get_profile.url())
                    .into_response()
                    .set_cookie(&SessionCookie::issued(session)),
                None => Response::text(403, "The signed-in subject is not a reader of the blog"),
            },
            Err(error) => Response::text(
                403,
                format!("The signed-in subject is not a reader of the blog: {error}"),
            ),
        }
    }
}
