use margaret_identity_session::id_token_claims::IdTokenClaims;

use crate::id_token_signing::IdTokenSigning;

pub enum ProviderIdentity {
    Asserted {
        claims: IdTokenClaims,
        signing: IdTokenSigning,
    },
    Withheld,
}
