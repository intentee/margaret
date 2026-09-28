use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;

pub enum AccessTokenSigning {
    ClaimsNotAnObject,
    CollidingClaim { member: String },
    Signed(AccessTokenClaimsSigned),
}
