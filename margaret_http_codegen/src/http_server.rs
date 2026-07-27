use proc_macro2::Ident;
use quote::format_ident;

use crate::server_transport_policy::ServerTransportPolicy;

#[derive(Clone)]
pub struct HttpServer {
    name: String,
    routes_are_async: bool,
    transport_policy: ServerTransportPolicy,
}

impl HttpServer {
    #[must_use]
    pub fn new(name: String, transport_policy: ServerTransportPolicy) -> Self {
        Self {
            name,
            routes_are_async: false,
            transport_policy,
        }
    }

    #[must_use]
    pub fn with_async_routes(mut self) -> Self {
        self.routes_are_async = true;
        self
    }

    #[must_use]
    pub fn address_argument(&self) -> String {
        format!("{}-addr", self.name)
    }

    #[must_use]
    pub fn function_name(&self) -> Ident {
        format_ident!("server_{}", self.name)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn routes_are_async(&self) -> bool {
        self.routes_are_async
    }

    #[must_use]
    pub fn transport_policy(&self) -> ServerTransportPolicy {
        self.transport_policy
    }

    #[must_use]
    pub fn upload_dir_argument(&self) -> String {
        format!("{}-upload-dir", self.name)
    }

    #[must_use]
    pub fn uploads_argument(&self) -> String {
        format!("{}-uploads", self.name)
    }

    #[must_use]
    pub fn url_argument(&self) -> String {
        format!("{}-url", self.name)
    }
}
