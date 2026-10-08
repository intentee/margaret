use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::stores::message_store::MessageStore;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/messages", server = "public")]
pub struct GetMessages {
    messages: Arc<MessageStore>,
}

impl GetMessages {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(messages: Arc<MessageStore>) -> anyhow::Result<Self> {
        Ok(Self { messages })
    }

    /// # Errors
    ///
    /// Returns an error when the messages cannot be read.
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::json(200, &self.messages.list().await?))
    }
}
