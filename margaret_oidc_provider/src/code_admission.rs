use url::Url;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;
use margaret_token_digest::equal_in_constant_time::equal_in_constant_time;

pub(crate) struct CodeAdmission<'admission> {
    pub(crate) client_id: &'admission ClientId,
    pub(crate) code_challenge: &'admission str,
    pub(crate) redirect_uri: &'admission Url,
}

impl CodeAdmission<'_> {
    pub(crate) fn admits(&self, grant: &AuthorizationGrant) -> bool {
        grant.client_id == *self.client_id
            && grant.redirect_uri == *self.redirect_uri
            && equal_in_constant_time(
                grant.code_challenge.as_bytes(),
                self.code_challenge.as_bytes(),
            )
    }
}
