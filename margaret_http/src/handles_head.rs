use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::request::Request;
use crate::response::Response;

#[async_trait]
pub trait HandlesHead: Send + Sync {
    async fn handle(&self, request: &Request) -> Result<Response, HandlerError>;
}
