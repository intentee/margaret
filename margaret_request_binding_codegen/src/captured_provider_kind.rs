use margaret_attributes::canonical_path::CanonicalPath;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::request_binding::RequestBinding;

pub enum CapturedProviderKind {
    AuthenticatedUser {
        application: AuthenticatedUserApplication,
    },
    Binder {
        accessor: String,
        provider: CanonicalPath,
    },
}

impl CapturedProviderKind {
    #[must_use]
    pub fn accessor(&self) -> &str {
        match self {
            Self::AuthenticatedUser { application } => &application.field,
            Self::Binder { accessor, .. } => accessor,
        }
    }

    #[must_use]
    pub fn binds(&self, binding: &RequestBinding) -> bool {
        match self {
            Self::AuthenticatedUser { application } => matches!(
                binding,
                RequestBinding::AuthenticatedUser { application: bound, .. }
                    if application.concrete == bound.concrete
            ),
            Self::Binder { provider, .. } => matches!(
                binding,
                RequestBinding::BoundRouteParameter { binder_provider, .. }
                    if provider == binder_provider
            ),
        }
    }
}
