use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::contract_revision::contract_revision;

pub fn beyond_the_database_range() -> SigningKeysRevision {
    SigningKeysRevision {
        generation: SigningKeysGeneration::new(u64::MAX),
        ..contract_revision(0)
    }
}
