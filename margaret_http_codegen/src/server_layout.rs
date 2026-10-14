use proc_macro2::Ident;

use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;
use crate::server_forwarding::ServerForwarding;

pub(crate) struct ServerLayout<'plan> {
    pub(crate) field: Ident,
    pub(crate) forwarding: ServerForwarding,
    pub(crate) named: Vec<NamedRoute<'plan>>,
    pub(crate) origin: Ident,
    pub(crate) paths: Ident,
    pub(crate) server: &'plan HttpServer,
    pub(crate) struct_ident: Ident,
}
