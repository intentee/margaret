use proc_macro2::Ident;
use quote::format_ident;
use syn::Path;

pub enum RequestInputSource {
    Cookie,
    Form,
    Query,
    Json,
}

impl RequestInputSource {
    pub(crate) fn from_path(path: &Path) -> Option<Self> {
        path.segments.last().and_then(|segment| {
            [Self::Cookie, Self::Form, Self::Query, Self::Json]
                .into_iter()
                .find(|source| segment.ident == source.written())
        })
    }

    pub(crate) fn variant(&self) -> Ident {
        format_ident!("{}", self.written())
    }

    pub(crate) fn written(&self) -> &'static str {
        match self {
            Self::Cookie => "Cookie",
            Self::Form => "Form",
            Self::Query => "Query",
            Self::Json => "Json",
        }
    }
}
