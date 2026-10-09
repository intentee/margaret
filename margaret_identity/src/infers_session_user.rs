use async_trait::async_trait;

use margaret_http::request::Request;

use crate::session_user_inference::SessionUserInference;

#[async_trait]
pub trait InfersSessionUser: Send + Sync {
    type User;

    async fn infer(&self, request: &Request) -> anyhow::Result<SessionUserInference<Self::User>>;
}
