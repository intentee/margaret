use async_trait::async_trait;

use margaret_http::request::Request;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

#[async_trait]
pub trait InfersAuthenticatedUser: Send + Sync {
    type User;

    async fn infer(&self, request: &Request) -> AuthenticatedUserOutcome<Self::User>;
}
