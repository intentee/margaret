use proc_macro2::Ident;
use quote::format_ident;
use syn::Path;

pub enum RequestInputSource {
    Form,
    Query,
    Json,
}

impl RequestInputSource {
    pub(crate) fn from_path(path: &Path) -> Option<Self> {
        let leaf = path.segments.last().map(|segment| &segment.ident);

        match leaf {
            Some(leaf) if leaf == "Form" => Some(Self::Form),
            Some(leaf) if leaf == "Query" => Some(Self::Query),
            Some(leaf) if leaf == "Json" => Some(Self::Json),
            _ => None,
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
