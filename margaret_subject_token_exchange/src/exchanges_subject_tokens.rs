use serde::de::DeserializeOwned;

use margaret_jwt_verification::verified_jwt::VerifiedJwt;

use crate::subject_token_exchange::SubjectTokenExchange;
use crate::subject_token_profile::SubjectTokenProfile;

pub trait ExchangesSubjectTokens: Send + Sync + 'static {
    type Claims: DeserializeOwned + Send + Sync;
    type Profile: SubjectTokenProfile;

    fn exchange(&self, token: &VerifiedJwt<Self::Claims, Self::Profile>) -> SubjectTokenExchange;
}
