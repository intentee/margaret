use syn::Path;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum RouteParameterBinding {
    CurrentRequest,
    OptionalActor,
    Raw {
        path_key: String,
    },
    RequiredActor,
    Bound {
        binder: CanonicalPath,
        intent: Option<Path>,
        path_key: String,
    },
}

impl RouteParameterBinding {
    pub(crate) fn is_actor(&self) -> bool {
        matches!(self, Self::RequiredActor | Self::OptionalActor)
    }

    pub(crate) fn is_bound(&self) -> bool {
        matches!(self, Self::Bound { .. })
    }

    pub(crate) fn is_authorizing(&self) -> bool {
        matches!(
            self,
            Self::Bound {
                intent: Some(_),
                ..
            }
        )
    }

    pub(crate) fn path_key(&self) -> Option<&str> {
        match self {
            Self::Raw { path_key } | Self::Bound { path_key, .. } => Some(path_key),
            _ => None,
        }
    }
}
