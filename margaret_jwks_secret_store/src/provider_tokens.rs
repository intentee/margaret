use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;

pub struct ProviderTokens {
    pub access_token: AccessTokenClaimsSigned,
    pub id_token: Option<String>,
}
