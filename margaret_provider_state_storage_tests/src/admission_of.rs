use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;
use margaret_provider_state_storage::code_admission::CodeAdmission;

#[must_use]
pub fn admission_of(grant: &AuthorizationGrant) -> CodeAdmission<'_> {
    CodeAdmission {
        client_id: &grant.client_id,
        code_challenge: &grant.code_challenge,
        redirect_uri: &grant.redirect_uri,
    }
}
