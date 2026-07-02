use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum RouteParameterBinding {
    CurrentRequest,
    Raw {
        path_key: String,
    },
    Bound {
        binder: CanonicalPath,
        path_key: String,
    },
}

impl RouteParameterBinding {
    pub(crate) fn is_bound(&self) -> bool {
        matches!(self, Self::Bound { .. })
    }

    pub(crate) fn path_key(&self) -> Option<&str> {
        match self {
            Self::Raw { path_key } | Self::Bound { path_key, .. } => Some(path_key),
            _ => None,
        }
    }
}
