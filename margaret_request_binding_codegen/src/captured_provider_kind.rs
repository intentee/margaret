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
        match (self, binding) {
            (
                Self::AuthenticatedUser { application },
                RequestBinding::AuthenticatedUser {
                    application: bound, ..
                },
            ) => application.field == bound.field,
            (Self::Binder { accessor, .. }, RequestBinding::Bound { binder_field, .. }) => {
                accessor == binder_field
            }
            _ => false,
        }
    }
}
