use proc_macro2::Ident;
use quote::format_ident;
use syn::Path;

pub(crate) enum RequestInputSource {
    Form,
    Query,
    Json,
}

impl RequestInputSource {
    pub(crate) fn from_path(path: &Path) -> Option<Self> {
        let leaf = &path
            .segments
            .last()
            .expect("a path has at least one segment")
            .ident;

        if leaf == "Form" {
            Some(Self::Form)
        } else if leaf == "Query" {
            Some(Self::Query)
        } else if leaf == "Json" {
            Some(Self::Json)
        } else {
            None
        }
    }

    pub(crate) fn variant(&self) -> Ident {
        match self {
            Self::Form => format_ident!("Form"),
            Self::Query => format_ident!("Query"),
            Self::Json => format_ident!("Json"),
        }
    }
}
