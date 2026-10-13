use proc_macro2::Ident;

use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;

pub(crate) struct ServerLayout<'plan> {
    pub(crate) constructor: Ident,
    pub(crate) field: Ident,
    pub(crate) named: Vec<NamedRoute<'plan>>,
    pub(crate) origin: Ident,
    pub(crate) paths: Ident,
    pub(crate) server: &'plan HttpServer,
    pub(crate) struct_ident: Ident,
}
