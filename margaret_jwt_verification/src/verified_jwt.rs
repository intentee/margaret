use margaret_jws_verification::key_id::KeyId;
use margaret_registered_claims::registered_claims::RegisteredClaims;

pub struct VerifiedJwt<TClaims> {
    pub claims: TClaims,
    pub kid: KeyId,
    pub registered: RegisteredClaims,
}
