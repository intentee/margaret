use margaret_attributes::canonical_path::CanonicalPath;

use crate::framework_service_kind::FrameworkServiceKind;

pub struct FrameworkService {
    pub concrete_path: CanonicalPath,
    pub field_name: String,
    pub is_async: bool,
    pub kind: FrameworkServiceKind,
    pub runner: String,
    pub takes_token: bool,
    pub type_name: String,
}
