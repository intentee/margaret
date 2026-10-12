use proc_macro2::Ident;

use crate::http_server::HttpServer;

pub(crate) struct ServerLayout<'server> {
    pub(crate) constructor: Ident,
    pub(crate) field: Ident,
    pub(crate) origin: Ident,
    pub(crate) paths: Ident,
    pub(crate) server: &'server HttpServer,
    pub(crate) struct_ident: Ident,
}
