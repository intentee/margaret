use margaret_attributes::canonical_path::CanonicalPath;

use crate::service_kind::ServiceKind;

pub(crate) struct ServiceUnit {
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) field_name: String,
    pub(crate) kind: ServiceKind,
    pub(crate) runner: String,
    pub(crate) takes_token: bool,
}

impl ServiceUnit {
    pub(crate) fn uses_token(&self) -> bool {
        matches!(self.kind, ServiceKind::Ticker { .. }) || self.takes_token
    }
}
