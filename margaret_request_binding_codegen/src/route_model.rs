use margaret_attributes::canonical_path::CanonicalPath;

use crate::route_model_key::RouteModelKey;

pub struct RouteModel {
    pub loaded: CanonicalPath,
    pub primary_key: RouteModelKey,
}
