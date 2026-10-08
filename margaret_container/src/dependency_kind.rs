use std::slice;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::url_source::UrlSource;

#[derive(Clone)]
pub(crate) enum DependencyKind {
    Collection { provider_keys: Vec<CanonicalPath> },
    Constant { path: CanonicalPath },
    ServeInput { input: Box<ServeInput> },
    Single { provider_key: CanonicalPath },
    Urls { sources: Vec<UrlSource> },
}

impl DependencyKind {
    pub(crate) fn provider_keys(&self) -> &[CanonicalPath] {
        match self {
            Self::Single { provider_key } => slice::from_ref(provider_key),
            Self::Collection { provider_keys } => provider_keys,
            Self::Constant { .. } | Self::ServeInput { .. } | Self::Urls { .. } => &[],
        }
    }
}
