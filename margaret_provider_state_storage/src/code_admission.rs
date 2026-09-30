use aws_lc_rs::constant_time::verify_slices_are_equal;
use url::Url;

use margaret_oauth_vocabulary::client_id::ClientId;

use crate::authorization_grant::AuthorizationGrant;

pub struct CodeAdmission<'admission> {
    pub client_id: &'admission ClientId,
    pub code_challenge: &'admission str,
    pub redirect_uri: &'admission Url,
}

impl CodeAdmission<'_> {
    pub(crate) fn admits(&self, grant: &AuthorizationGrant) -> bool {
        grant.client_id == *self.client_id
            && grant.redirect_uri == *self.redirect_uri
            && verify_slices_are_equal(
                grant.code_challenge.as_bytes(),
                self.code_challenge.as_bytes(),
            )
            .is_ok()
    }
}
