use proc_macro2::Ident;
use quote::format_ident;

use margaret_http_codegen::http_server::HttpServer;

pub(crate) struct ServerBindings {
    pub(crate) address: Ident,
    pub(crate) listener: Ident,
    pub(crate) transport: Ident,
    pub(crate) upload_directory: Ident,
}

impl ServerBindings {
    pub(crate) fn of(server: &HttpServer) -> Self {
        let function_name = server.function_name();

        Self {
            address: format_ident!("{function_name}_address"),
            listener: format_ident!("{function_name}_listener"),
            transport: format_ident!("{function_name}_transport"),
            upload_directory: format_ident!("{function_name}_upload_directory"),
        }
    }
}
