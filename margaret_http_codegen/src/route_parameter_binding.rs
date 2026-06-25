use margaret_attributes::canonical_path::CanonicalPath;

use crate::authorization::Authorization;

pub(crate) enum RouteParameterBinding {
    Raw,
    Bound {
        binder: CanonicalPath,
        authorization: Option<Authorization>,
    },
}
