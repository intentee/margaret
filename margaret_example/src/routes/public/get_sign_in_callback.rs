use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::active_record::creation::Creation;
use margaret::framework::database::database::Database;
use margaret::framework::http::redirect::Redirect;
use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret::framework::oidc_sign_in::signed_in::SignedIn;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::sign_in_claims::SignInClaims;
use crate::forms::session_cookie::SessionCookie;
use crate::margaret::oauth_clients::blog::SignInFlow;
use crate::margaret::routes::Routes;
use crate::models::user_session::UserSession;
use crate::system_clock::SystemClock;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/sign-in/callback", server = "public")]
pub struct GetSignInCallback {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
    sign_in_flow: Arc<SignInFlow>,
}

impl GetSignInCallback {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        clock: Arc<SystemClock>,
        database: Arc<Database>,
        sign_in_flow: Arc<SignInFlow>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            clock,
            database,
            sign_in_flow,
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
                    .await?
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

    async fn signed_in(&self, subject: &str, routes: &Routes) -> anyhow::Result<Response> {
        Ok(match Uuid::parse_str(subject) {
            Ok(user_id) => {
                match UserSession::start(&self.database, user_id, self.clock.now()).await? {
                    Creation::Created(session) => {
                        Redirect::see_other(routes.public.get_profile.url())
                            .into_response()
                            .set_cookie(&SessionCookie::issued(session.id))
                    }
                    Creation::Refused => {
                        Response::text(403, "The signed-in subject is not a reader of the blog")
                    }
                }
            }
            Err(error) => Response::text(
                403,
                format!("The signed-in subject is not a reader of the blog: {error}"),
            ),
        })
    }
}
