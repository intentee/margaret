use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use async_trait::async_trait;
use margaret_http::cookie::Cookie;
use margaret_http::cookie::SameSite;
use margaret_http::cookie::time::Duration;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::provides_authenticated_actor;
use margaret_macros::singleton;
use margaret_security::actor::Actor;
use margaret_security::authenticated_actor::AuthenticatedActor;
use margaret_security::authenticated_actor_store::AuthenticatedActorStore;
use margaret_security::user_repository::UserRepository as UserRepositoryContract;

use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use crate::security::access_token_claims::AccessTokenClaims;
use crate::security::signs_token_claims::SignsTokenClaims;
use crate::security::verifies_token::VerifiesToken;

const COOKIE_NAME: &str = "access_token";
const LIFETIME_SECONDS: i64 = 15 * 60;

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is after the unix epoch")
        .as_secs() as i64
}

fn active_cookie(token: String) -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, token))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .build()
}

fn expired_cookie() -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, String::new()))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .max_age(Duration::ZERO)
        .build()
}

#[singleton]
#[provides_authenticated_actor]
pub struct TokenActorStore {
    repository: Arc<UserRepository>,
    signer: Arc<dyn SignsTokenClaims>,
    verifier: Arc<dyn VerifiesToken>,
}

impl TokenActorStore {
    #[constructor]
    pub fn create(
        repository: Arc<UserRepository>,
        signer: Arc<dyn SignsTokenClaims>,
        verifier: Arc<dyn VerifiesToken>,
    ) -> Self {
        Self {
            repository,
            signer,
            verifier,
        }
    }

    pub fn login(&self, response: Response, user: &User) -> Response {
        let issued_at = unix_now();
        let token = self.signer.sign(&AccessTokenClaims {
            exp: issued_at + LIFETIME_SECONDS,
            iat: issued_at,
            sub: user.identifier().to_string(),
        });

        response.set_cookie(active_cookie(token))
    }

    pub fn logout(&self, response: Response) -> Response {
        response.set_cookie(expired_cookie())
    }
}

#[async_trait]
impl AuthenticatedActorStore for TokenActorStore {
    type Actor = User;

    async fn get_authenticated_actor(&self, request: &Request) -> AuthenticatedActor<User> {
        let user = match request.cookie(COOKIE_NAME) {
            Some(token) => match self.verifier.verify(&token) {
                Some(claims) => self.repository.find_user_by_id(&claims.sub).await,
                None => None,
            },
            None => None,
        };

        match user {
            Some(user) => AuthenticatedActor::Session(user),
            None => AuthenticatedActor::Anonymous,
        }
    }
}
