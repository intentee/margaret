use margaret_attributes::canonical_path::CanonicalPath;

use crate::framework_service::FrameworkService;
use crate::service_kind::ServiceKind;
use crate::service_unit_origin::ServiceUnitOrigin;

pub(crate) struct ServiceUnit {
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) field_name: String,
    pub(crate) is_async: bool,
    pub(crate) kind: ServiceKind,
    pub(crate) origin: ServiceUnitOrigin,
    pub(crate) runner: String,
    pub(crate) takes_token: bool,
    pub(crate) type_name: String,
}

impl ServiceUnit {
    pub(crate) fn from_framework(
        FrameworkService {
            concrete_path,
            field_name,
            is_async,
            outcome,
            runner,
            takes_token,
            type_name,
        }: &FrameworkService,
    ) -> Self {
        Self {
            concrete_path: concrete_path.clone(),
            field_name: field_name.clone(),
            is_async: *is_async,
            kind: ServiceKind::Service,
            origin: ServiceUnitOrigin::Framework { outcome: *outcome },
            runner: runner.clone(),
            takes_token: *takes_token,
            type_name: type_name.clone(),
        }
    }
}
