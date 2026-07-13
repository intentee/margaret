use proc_macro2::Ident;
use quote::format_ident;

use crate::server_transport_policy::ServerTransportPolicy;

#[derive(Clone)]
pub struct HttpServer {
    name: String,
    transport_policy: ServerTransportPolicy,
}

impl HttpServer {
    pub fn new(name: String, transport_policy: ServerTransportPolicy) -> Self {
        Self {
            name,
            transport_policy,
        }
    }

    pub fn address_argument(&self) -> String {
        format!("{}-addr", self.name)
    }

    pub fn function_name(&self) -> Ident {
        format_ident!("server_{}", self.name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn transport_argument(&self) -> String {
        format!("{}-transport", self.name)
    }

    pub fn transport_policy(&self) -> ServerTransportPolicy {
        self.transport_policy
    }

    pub fn upload_dir_argument(&self) -> String {
        format!("{}-upload-dir", self.name)
    }

    pub fn uploads_argument(&self) -> String {
        format!("{}-uploads", self.name)
    }

    pub fn url_argument(&self) -> String {
        format!("{}-url", self.name)
    }
}
