use std::collections::HashSet;

use crate::bound_credentials::BoundCredentials;

pub(crate) struct BoundArguments {
    pub(crate) credentials: BoundCredentials,
    pub(crate) route_parameters: HashSet<String>,
}
