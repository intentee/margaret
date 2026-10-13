use async_trait::async_trait;

use margaret::framework::macros::admits_sign_in;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_sign_in::admits_sign_in::AdmitsSignIn;
use margaret::framework::oidc_sign_in::sign_in_admission::SignInAdmission;
use margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow;
use margaret::framework::oidc_sign_in::signed_in::SignedIn;

use crate::partner_claims::PartnerClaims;

#[singleton]
#[admits_sign_in(client = partner_client)]
pub struct PartnerReaders;

#[async_trait]
impl AdmitsSignIn for PartnerReaders {
    type IdClaims = PartnerClaims;

    async fn admit(
        &self,
        SignedIn { claims, .. }: SignedIn<PartnerClaims>,
        _flow: &SignInFlow,
    ) -> anyhow::Result<SignInAdmission> {
        Ok(if claims.verified_reader {
            SignInAdmission::Admitted(claims.reader)
        } else {
            SignInAdmission::Refused
        })
    }
}
