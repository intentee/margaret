use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;

pub enum SessionRefresh {
    Refreshed(AccessTokenClaimsSigned),
    Refused,
}
