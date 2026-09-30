use std::ops::ControlFlow;

use margaret_oauth_vocabulary::client_id::ClientId;

use crate::refresh_family::RefreshFamily;
use crate::refresh_rotation::RefreshRotation;
use crate::refresh_scope::RefreshScope;

pub struct RefreshAdmission<'admission> {
    pub client_id: &'admission ClientId,
    pub scope: &'admission RefreshScope,
}

impl RefreshAdmission<'_> {
    pub(crate) fn admits(&self, family: &RefreshFamily) -> ControlFlow<RefreshRotation> {
        if family.client_id != *self.client_id {
            ControlFlow::Break(RefreshRotation::ForeignClient)
        } else if self.scope.is_within(&family.scopes) {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(RefreshRotation::ScopeExceeded)
        }
    }
}
