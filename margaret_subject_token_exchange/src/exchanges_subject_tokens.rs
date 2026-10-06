use async_trait::async_trait;
use serde::de::DeserializeOwned;

use margaret_jwt_verification::verified_jwt::VerifiedJwt;

use crate::subject_token_exchange::SubjectTokenExchange;
use crate::subject_token_profile::SubjectTokenProfile;

#[async_trait]
pub trait ExchangesSubjectTokens: Send + Sync + 'static {
    type Claims: DeserializeOwned + Send + Sync;
    type Profile: SubjectTokenProfile + Send + Sync;

    async fn exchange(
        &self,
        token: &VerifiedJwt<Self::Claims, Self::Profile>,
    ) -> anyhow::Result<SubjectTokenExchange>;
}
