use async_trait::async_trait;
use uuid::Uuid;

use margaret_oidc_sign_in::admits_sign_in::AdmitsSignIn;
use margaret_oidc_sign_in::sign_in_admission::SignInAdmission;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::signed_in::SignedIn;

use crate::email_claims::EmailClaims;

pub enum FixtureAdmission {
    Admitting(Uuid),
    Failing,
    Refusing,
}

#[async_trait]
impl AdmitsSignIn for FixtureAdmission {
    type IdClaims = EmailClaims;

    async fn admit(
        &self,
        _signed_in: SignedIn<EmailClaims>,
        _flow: &SignInFlow,
    ) -> anyhow::Result<SignInAdmission> {
        match self {
            Self::Admitting(subject) => Ok(SignInAdmission::Admitted(*subject)),
            Self::Failing => Err(anyhow::anyhow!("the admission store is unavailable")),
            Self::Refusing => Ok(SignInAdmission::Refused),
        }
    }
}
