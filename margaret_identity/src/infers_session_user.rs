use async_trait::async_trait;

use margaret_http::request::Request;

#[async_trait]
pub trait InfersSessionUser: Send + Sync {
    type Inference;

    async fn infer(&self, request: &Request) -> Self::Inference;
}
