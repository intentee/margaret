use syn::Path;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::session_requirement::SessionRequirement;

pub(crate) enum RouteParameterBinding {
    CurrentRequest,
    Raw,
    SessionAuthenticated(SessionRequirement),
    Bound {
        binder: CanonicalPath,
        intent: Option<Path>,
    },
}
