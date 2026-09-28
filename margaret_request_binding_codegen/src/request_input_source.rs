use proc_macro2::Ident;
use quote::format_ident;

use margaret_attributes::canonical_path::CanonicalPath;

const REQUEST_INPUT_PATH: [&str; 5] = [
    "margaret",
    "framework",
    "http_validation",
    "request_input",
    "RequestInput",
];

pub enum RequestInputSource {
    Cookie,
    Form,
    Query,
    Json,
}

impl RequestInputSource {
    pub(crate) fn from_canonical(resolved: &CanonicalPath) -> Option<Self> {
        [Self::Cookie, Self::Form, Self::Query, Self::Json]
            .into_iter()
            .find(|source| &source.canonical_path() == resolved)
    }

    pub(crate) fn variant(&self) -> Ident {
        match self {
            Self::Cookie => format_ident!("Cookie"),
            Self::Form => format_ident!("Form"),
            Self::Query => format_ident!("Query"),
            Self::Json => format_ident!("Json"),
        }
    }

    fn canonical_path(&self) -> CanonicalPath {
        let mut segments: Vec<String> =
            REQUEST_INPUT_PATH.iter().map(ToString::to_string).collect();

        segments.push(self.variant().to_string());

        CanonicalPath::new(segments)
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::RequestInputSource;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    fn request_input(variant: &str) -> CanonicalPath {
        path(&[
            "margaret",
            "framework",
            "http_validation",
            "request_input",
            "RequestInput",
            variant,
        ])
    }

    #[test]
    fn recognises_every_request_input_variant_by_its_canonical_path() {
        let recognised: Vec<String> = ["Cookie", "Form", "Query", "Json"]
            .into_iter()
            .filter_map(|variant| RequestInputSource::from_canonical(&request_input(variant)))
            .map(|source| source.variant().to_string())
            .collect();

        assert_eq!(recognised, vec!["Cookie", "Form", "Query", "Json"]);
    }

    #[test]
    fn rejects_a_same_named_variant_of_a_foreign_enum() {
        assert!(
            RequestInputSource::from_canonical(&path(&[
                "crate",
                "inputs",
                "RequestInput",
                "Cookie"
            ]))
            .is_none()
        );
    }
}
