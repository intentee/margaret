use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::sign_in_admission::SignInAdmission;
use crate::sign_in_flow::SignInFlow;
use crate::signed_in::SignedIn;

#[async_trait]
pub trait AdmitsSignIn: Send + Sync {
    type IdClaims: DeserializeOwned + Send;

    async fn admit(
        &self,
        signed_in: SignedIn<Self::IdClaims>,
        flow: &SignInFlow,
    ) -> anyhow::Result<SignInAdmission>;
}
