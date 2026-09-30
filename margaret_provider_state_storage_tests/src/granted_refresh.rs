use margaret_provider_state_storage::refresh_admission::RefreshAdmission;
use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_provider_state_storage::refresh_scope::RefreshScope;

#[must_use]
pub fn granted_refresh(family: &RefreshFamily) -> RefreshAdmission<'_> {
    RefreshAdmission {
        client_id: &family.client_id,
        scope: &RefreshScope::Granted,
    }
}
