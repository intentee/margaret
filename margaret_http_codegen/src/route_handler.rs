use proc_macro2::Ident;
use proc_macro2::TokenStream;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum RouteHandler {
    Responder {
        field: Ident,
        path: CanonicalPath,
    },
    Synthetic {
        handler: TokenStream,
        label: String,
    },
}

impl RouteHandler {
    pub(crate) fn describe(&self) -> String {
        match self {
            Self::Responder { path, .. } => path.to_string(),
            Self::Synthetic { label, .. } => label.clone(),
        }
    }
}
