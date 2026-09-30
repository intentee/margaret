use std::slice;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::serve_input::ServeInput;

#[derive(Clone)]
pub(crate) enum DependencyKind {
    Borrowed {
        provider_key: CanonicalPath,
    },
    Collection {
        provider_keys: Vec<CanonicalPath>,
    },
    Constant {
        path: CanonicalPath,
    },
    ServeInput {
        input: Box<ServeInput>,
    },
    Single {
        provider_key: CanonicalPath,
    },
    ViewCollection {
        provider_keys: Vec<CanonicalPath>,
        view: CanonicalPath,
    },
}

impl DependencyKind {
    pub(crate) fn provider_keys(&self) -> &[CanonicalPath] {
        match self {
            Self::Borrowed { provider_key } | Self::Single { provider_key } => {
                slice::from_ref(provider_key)
            }
            Self::Collection { provider_keys } | Self::ViewCollection { provider_keys, .. } => {
                provider_keys
            }
            Self::Constant { .. } | Self::ServeInput { .. } => &[],
        }
    }
}
