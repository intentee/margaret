use std::collections::HashMap;

use async_trait::async_trait;
use bytes::Bytes;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_sync_holder::sync_holder::SyncHolder;

#[derive(Default)]
pub struct PublishedDocuments {
    documents: HashMap<String, SyncHolder<Bytes>>,
}

impl PublishedDocuments {
    pub fn publish(&mut self, path: String, document: SyncHolder<Bytes>) {
        self.documents.insert(path, document);
    }
}

#[async_trait]
impl HeadHandler for PublishedDocuments {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        let path = format!(
            "/{}",
            request
                .path_param("document")
                .expect("the document path is captured")
        );

        Ok(ResponseContinuation::Done(
            self.documents
                .get(&path)
                .map_or_else(Response::not_found, |document| {
                    Response::bytes(200, "application/json", document.get())
                }),
        ))
    }
}
