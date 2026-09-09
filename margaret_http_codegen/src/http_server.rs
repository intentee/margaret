use proc_macro2::Ident;
use quote::format_ident;

use crate::server_transport_policy::ServerTransportPolicy;

#[derive(Clone)]
pub struct HttpServer {
    address_argument: String,
    body_limit_argument: String,
    name: String,
    transport_policy: ServerTransportPolicy,
    transport_argument: String,
    upload_dir_argument: String,
    uploads_argument: String,
    url_argument: String,
}

impl HttpServer {
    #[must_use]
    pub fn new(name: String, transport_policy: ServerTransportPolicy) -> Self {
        let address_argument = format!("{name}-addr");
        let body_limit_argument = format!("{name}-body-limit");
        let transport_argument = format!("{name}-transport");
        let upload_dir_argument = format!("{name}-upload-dir");
        let uploads_argument = format!("{name}-uploads");
        let url_argument = format!("{name}-url");

        Self {
            address_argument,
            body_limit_argument,
            name,
            transport_policy,
            transport_argument,
            upload_dir_argument,
            uploads_argument,
            url_argument,
        }
    }

    #[must_use]
    pub fn address_argument(&self) -> &str {
        &self.address_argument
    }

    #[must_use]
    pub fn body_limit_argument(&self) -> &str {
        &self.body_limit_argument
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
    pub fn transport_argument(&self) -> &str {
        &self.transport_argument
    }

    #[must_use]
    pub fn transport_policy(&self) -> ServerTransportPolicy {
        self.transport_policy
    }

    #[must_use]
    pub fn upload_dir_argument(&self) -> &str {
        &self.upload_dir_argument
    }

    #[must_use]
    pub fn uploads_argument(&self) -> &str {
        &self.uploads_argument
    }

    #[must_use]
    pub fn url_argument(&self) -> &str {
        &self.url_argument
    }
}
