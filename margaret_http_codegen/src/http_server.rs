use proc_macro2::Ident;
use quote::format_ident;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::server_origin_source::ServerOriginSource;
use crate::server_transport_policy::ServerTransportPolicy;
use crate::server_uploads::ServerUploads;

#[derive(Clone)]
pub struct HttpServer {
    name: String,
    origin: ServerOriginSource,
    transport_policy: ServerTransportPolicy,
    uploads: ServerUploads,
}

impl HttpServer {
    #[must_use]
    pub fn new(
        name: String,
        transport_policy: ServerTransportPolicy,
        uploads: ServerUploads,
    ) -> Self {
        Self {
            name,
            origin: ServerOriginSource::Argument,
            transport_policy,
            uploads,
        }
    }

    #[must_use]
    pub fn served_at_issuer_origin(self, endpoints: CanonicalPath) -> Self {
        Self {
            origin: ServerOriginSource::Issuer { endpoints },
            ..self
        }
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
    pub fn origin(&self) -> &ServerOriginSource {
        &self.origin
    }

    #[must_use]
    pub fn transport_argument(&self) -> String {
        format!("{}-transport", self.name)
    }

    #[must_use]
    pub fn transport_policy(&self) -> ServerTransportPolicy {
        self.transport_policy
    }

    #[must_use]
    pub fn uploads(&self) -> ServerUploads {
        self.uploads
    }

    #[must_use]
    pub fn upload_dir_argument(&self) -> String {
        format!("{}-upload-dir", self.name)
    }

    #[must_use]
    pub fn url_argument(&self) -> String {
        format!("{}-url", self.name)
    }
}
