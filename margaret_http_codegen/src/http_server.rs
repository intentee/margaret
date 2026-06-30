use proc_macro2::Ident;
use quote::format_ident;

pub struct HttpServer {
    name: String,
}

impl HttpServer {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn function_name(&self) -> Ident {
        format_ident!("server_{}", self.name)
    }

    pub fn address_argument(&self) -> String {
        format!("{}-addr", self.name)
    }
}
