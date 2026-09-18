use proc_macro2::Ident;
use quote::format_ident;

use crate::server_name::ServerName;
use crate::server_transport_policy::ServerTransportPolicy;

#[derive(Clone)]
pub struct HttpServer {
    name: ServerName,
    serves_web_socket_routes: bool,
    transport_policy: ServerTransportPolicy,
}

impl HttpServer {
    pub(crate) fn new(
        name: ServerName,
        serves_web_socket_routes: bool,
        transport_policy: ServerTransportPolicy,
    ) -> Self {
        Self {
            name,
            serves_web_socket_routes,
            transport_policy,
        }
    }

    #[must_use]
    pub fn address_argument(&self) -> String {
        format!("{}-addr", self.name)
    }

    #[must_use]
    pub fn function_name(&self) -> Ident {
        format_ident!("server_{}", self.name.as_str())
    }

    #[must_use]
    pub fn name(&self) -> &ServerName {
        &self.name
    }

    #[must_use]
    pub fn serves_web_socket_routes(&self) -> bool {
        self.serves_web_socket_routes
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

#[cfg(test)]
mod tests {
    use super::HttpServer;
    use crate::server_name::ServerName;
    use crate::server_transport_policy::ServerTransportPolicy;

    fn public() -> HttpServer {
        HttpServer::new(
            ServerName::parse("public".to_string()).expect("the fixture name is snake_case"),
            false,
            ServerTransportPolicy::Negotiable,
        )
    }

    #[test]
    fn derives_the_console_arguments_from_the_name() {
        let server = public();

        assert_eq!(server.address_argument(), "public-addr");
        assert_eq!(server.transport_argument(), "public-transport");
        assert_eq!(server.upload_dir_argument(), "public-upload-dir");
        assert_eq!(server.uploads_argument(), "public-uploads");
        assert_eq!(server.url_argument(), "public-url");
    }

    #[test]
    fn derives_the_route_function_name_from_the_name() {
        assert_eq!(public().function_name().to_string(), "server_public");
    }

    #[test]
    fn exposes_the_name_it_was_assembled_from() {
        assert_eq!(public().name().as_str(), "public");
    }
}
